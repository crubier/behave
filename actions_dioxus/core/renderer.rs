//! BehaviorTreeRenderer -- the Dioxus custom renderer that executes
//! behavior trees.
//!
//! Three data flows managed by the renderer:
//!
//! - **Output** (up): each tick, polls `behavior.output()` and stores
//!   it in the node's `Run` record.
//! - **Result** (up): on completion, calls `behavior.result()` and
//!   finalizes the `Run`. Child runs are attached to parent runs.
//! - **Input** (down): `send_input()` delivers serialized protobuf input
//!   to a running node, which decodes it into its typed `Input` struct.
//!
//! Uses sparse `Vec<Option<_>>` for O(1) node lookup by `ElementId`,
//! as recommended by the Dioxus docs. No HashMap.

use dioxus_core::{AttributeValue, ElementId, Template, TemplateNode, WriteMutations};
use log::{info, warn};

use crate::actions_dioxus::ActionIO;

use super::behavior::{ChildResult, NodeBehavior, NodeResponse};
use super::data::{now_utime, ProtoBytes, Run, RunId, RunStatus};
use super::node::{NodeState, RendererNode};

// ── Node factory ────────────────────────────────────────────────

fn create_node(tag: &str, args: &[u8]) -> Option<Box<dyn NodeBehavior>> {
    match tag {
        "sequence" => Some(Box::new(crate::actions_dioxus::sequence::SequenceNode::from_bytes(args))),
        "fallback" => Some(Box::new(crate::actions_dioxus::fallback::FallbackNode::from_bytes(args))),
        "takeoff"  => Some(Box::new(crate::actions_dioxus::takeoff::TakeoffNode::from_bytes(args))),
        "land"     => Some(Box::new(crate::actions_dioxus::land::LandNode::from_bytes(args))),
        "goto"     => Some(Box::new(crate::actions_dioxus::goto_waypoint::GotoWaypointNode::from_bytes(args))),
        "home"     => Some(Box::new(crate::actions_dioxus::home::HomeNode::from_bytes(args))),
        "photo"    => Some(Box::new(crate::actions_dioxus::take_photo::TakePhotoNode::from_bytes(args))),
        "parallel" => Some(Box::new(crate::actions_dioxus::parallel::ParallelNode::from_bytes(args))),
        _ => None,
    }
}

// ── Sparse node storage ─────────────────────────────────────────

/// Sparse vec indexed by `ElementId.0`. Grows as needed, O(1) access.
struct SparseNodes {
    slots: Vec<Option<RendererNode>>,
}

impl SparseNodes {
    fn new() -> Self {
        Self { slots: Vec::new() }
    }

    fn get(&self, id: ElementId) -> Option<&RendererNode> {
        self.slots.get(id.0)?.as_ref()
    }

    fn get_mut(&mut self, id: ElementId) -> Option<&mut RendererNode> {
        self.slots.get_mut(id.0)?.as_mut()
    }

    fn insert(&mut self, id: ElementId, node: RendererNode) {
        if id.0 >= self.slots.len() {
            self.slots.resize_with(id.0 + 1, || None);
        }
        self.slots[id.0] = Some(node);
    }

    fn remove(&mut self, id: ElementId) -> Option<RendererNode> {
        if id.0 < self.slots.len() {
            self.slots[id.0].take()
        } else {
            None
        }
    }

    fn count(&self) -> usize {
        self.slots.iter().filter(|s| s.is_some()).count()
    }

    /// Iterate over all occupied slots with their ElementId.
    fn iter(&self) -> impl Iterator<Item = (ElementId, &RendererNode)> {
        self.slots.iter().enumerate().filter_map(|(i, slot)| {
            slot.as_ref().map(|n| (ElementId(i), n))
        })
    }

    /// Iterate mutably over all occupied slots with their ElementId.
    fn iter_mut(&mut self) -> impl Iterator<Item = (ElementId, &mut RendererNode)> {
        self.slots.iter_mut().enumerate().filter_map(|(i, slot)| {
            slot.as_mut().map(|n| (ElementId(i), n))
        })
    }
}

// ── Renderer ────────────────────────────────────────────────────

pub struct BehaviorTreeRenderer {
    /// Sparse vec of nodes, indexed by Dioxus ElementId.
    nodes: SparseNodes,

    /// Dioxus mutation stack.
    stack: Vec<ElementId>,

    /// Root element ID.
    root: Option<ElementId>,

    /// Completed mission run (set when root completes).
    mission_run: Option<Run>,

    /// Monotonic run ID counter.
    next_run_id: RunId,

    /// Counter for temporary template-internal IDs.
    next_temp_id: usize,
}

impl BehaviorTreeRenderer {
    pub fn new() -> Self {
        Self {
            nodes: SparseNodes::new(),
            stack: Vec::new(),
            root: None,
            mission_run: None,
            next_run_id: 1,
            next_temp_id: 1_000_000,
        }
    }

    fn alloc_run_id(&mut self) -> RunId {
        let id = self.next_run_id;
        self.next_run_id += 1;
        id
    }

    /// Take the completed mission run, if the root has finished.
    pub fn take_mission_run(&mut self) -> Option<Run> {
        self.mission_run.take()
    }

    /// Get a snapshot of the current run tree (for real-time reporting).
    pub fn snapshot(&self) -> Option<Run> {
        self.root.and_then(|id| self.build_run_snapshot(id))
    }

    /// Send a serialized protobuf input to a specific node.
    pub fn send_input(&mut self, id: ElementId, input: &[u8]) {
        if let Some(node) = self.nodes.get_mut(id) {
            if let NodeState::Active(ref mut behavior) = node.state {
                behavior.set_input_bytes(input);
            }
        }
    }

    /// Send a serialized protobuf input to all active nodes with the given tag.
    pub fn send_input_by_tag(&mut self, tag: &str, input: &[u8]) {
        let ids: Vec<ElementId> = self.nodes.iter()
            .filter(|(_, n)| n.tag == tag && n.is_active())
            .map(|(id, _)| id)
            .collect();
        for id in ids {
            self.send_input(id, input);
        }
    }

    /// Number of tracked nodes.
    pub fn node_count(&self) -> usize {
        self.nodes.count()
    }

    // ── Run snapshots ───────────────────────────────────────────

    fn build_run_snapshot(&self, id: ElementId) -> Option<Run> {
        let node = self.nodes.get(id)?;
        let mut run = node.run.clone()?;

        run.children.clear();
        for &child_id in &node.children {
            if let Some(child_run) = self.build_run_snapshot(child_id) {
                run.children.push(child_run);
            }
        }

        Some(run)
    }

    // ── Child resolution ──────────────────────────────────────

    /// Get effective children, flattening through placeholder/text nodes.
    ///
    /// Dioxus component wrappers insert placeholder nodes for `{children}`.
    /// This resolves through them so the behavior tree sees the real children.
    fn effective_children(&self, id: ElementId) -> Vec<ElementId> {
        let node = match self.nodes.get(id) {
            Some(n) => n,
            None => return Vec::new(),
        };
        let mut result = Vec::new();
        for &child_id in &node.children {
            if let Some(child) = self.nodes.get(child_id) {
                if child.tag == "placeholder" || child.tag == "text" {
                    // Transparent: recurse into its children
                    result.extend(self.effective_children(child_id));
                } else {
                    result.push(child_id);
                }
            }
        }
        result
    }

    // ── Unified response handling ───────────────────────────────

    fn handle_response(&mut self, id: ElementId, response: NodeResponse, io: &ActionIO) {
        match response {
            NodeResponse::Running => {}
            NodeResponse::ActivateChild(index) => {
                let children = self.effective_children(id);
                if let Some(&cid) = children.get(index) {
                    self.activate(cid, io);
                } else {
                    warn!("ActivateChild({index}) out of bounds ({} effective children)", children.len());
                }
            }
            NodeResponse::ActivateAllChildren => {
                let child_ids = self.effective_children(id);
                for cid in child_ids {
                    self.activate(cid, io);
                }
            }
            NodeResponse::Success => {
                self.complete_node(id, RunStatus::Succeeded);
                self.propagate_up(id, ChildResult::Success, io);
            }
            NodeResponse::Failure => {
                self.complete_node(id, RunStatus::Failed);
                self.propagate_up(id, ChildResult::Failure, io);
            }
        }
    }

    // ── Activation / completion ─────────────────────────────────

    pub fn activate(&mut self, id: ElementId, io: &ActionIO) {
        let (tag, args) = match self.nodes.get(id) {
            Some(n) => (n.tag, n.args_bytes.clone()),
            None => return,
        };

        if let Some(mut behavior) = create_node(tag, &args) {
            let run_id = self.alloc_run_id();
            let now = now_utime();
            info!("[{tag}] activate (run #{run_id})");

            let response = behavior.on_activate(io);

            let node = self.nodes.get_mut(id).unwrap();
            node.start_run(run_id, now);
            node.state = NodeState::Active(behavior);

            self.handle_response(id, response, io);
        } else {
            warn!("[{tag}] unknown element");
        }
    }

    fn complete_node(&mut self, id: ElementId, status: RunStatus) {
        let now = now_utime();

        // Collect child runs first (separate borrow scope)
        let child_runs: Vec<Run> = self.nodes.get(id)
            .map(|n| {
                n.children.iter()
                    .filter_map(|&cid| self.nodes.get(cid).and_then(|cn| cn.run.clone()))
                    .collect()
            })
            .unwrap_or_default();

        if let Some(node) = self.nodes.get_mut(id) {
            let (result, state) = if let NodeState::Active(ref behavior) = node.state {
                (behavior.result_bytes(), behavior.state_bytes())
            } else {
                (Vec::new(), Vec::new())
            };

            if let Some(ref mut run) = node.run {
                run.complete(status, result, state, now);
                run.children = child_runs;
            }

            node.state = NodeState::Completed(match status {
                RunStatus::Succeeded => ChildResult::Success,
                RunStatus::Failed => ChildResult::Failure,
                RunStatus::Running => ChildResult::Success,
            });
        }
    }

    pub fn deactivate(&mut self, id: ElementId) {
        if let Some(node) = self.nodes.get(id) {
            let children: Vec<_> = node.children.clone();
            for child_id in children {
                self.deactivate(child_id);
            }
        }
        if let Some(node) = self.nodes.get_mut(id) {
            if node.is_active() {
                info!("[{}] deactivate", node.tag);
            }
            node.state = NodeState::Idle;
        }
    }

    // ── Result propagation ──────────────────────────────────────

    /// Find the nearest non-placeholder ancestor.
    fn effective_parent(&self, id: ElementId) -> Option<ElementId> {
        let mut current = self.nodes.get(id)?.parent?;
        loop {
            let node = self.nodes.get(current)?;
            if node.tag != "placeholder" && node.tag != "text" {
                return Some(current);
            }
            current = node.parent?;
        }
    }

    fn propagate_up(&mut self, child_id: ElementId, result: ChildResult, io: &ActionIO) {
        let parent_id = match self.effective_parent(child_id) {
            Some(pid) => pid,
            None => {
                if let Some(node) = self.nodes.get(child_id) {
                    if let Some(run) = &node.run {
                        info!("mission {:?} (run #{})", run.status, run.id);
                        self.mission_run = Some(run.clone());
                    }
                }
                return;
            }
        };

        let effective = self.effective_children(parent_id);
        let child_index = effective.iter().position(|&c| c == child_id).unwrap_or(0);
        let child_count = effective.len();

        let response = {
            let parent = match self.nodes.get_mut(parent_id) {
                Some(p) => p,
                None => return,
            };
            match &mut parent.state {
                NodeState::Active(ref mut behavior) => {
                    behavior.on_child_complete(child_index, child_count, result)
                }
                _ => return,
            }
        };

        self.handle_response(parent_id, response, io);
    }

    // ── Tick loop ───────────────────────────────────────────────

    pub fn tick_actions(&mut self, io: &ActionIO) {
        // 1. Update output + state for all active nodes
        for (_, node) in self.nodes.iter_mut() {
            if let NodeState::Active(ref behavior) = node.state {
                if let Some(ref mut run) = node.run {
                    run.output = behavior.output_bytes();
                    run.state = behavior.state_bytes();
                }
            }
        }

        // 2. Collect tick responses
        let responses: Vec<(ElementId, NodeResponse)> = self.nodes.iter_mut()
            .filter_map(|(id, node)| {
                if let NodeState::Active(ref mut behavior) = node.state {
                    let response = behavior.on_tick(io);
                    if response != NodeResponse::Running {
                        Some((id, response))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();

        // 3. Dispatch
        for (id, response) in responses {
            self.handle_response(id, response, io);
        }
    }

    /// Activate the root after `dom.rebuild()`.
    ///
    /// The root is the parentless element with the lowest ElementId.
    pub fn activate_root(&mut self, io: &ActionIO) {
        let root_id = self.nodes.iter()
            .filter(|(_, n)| n.parent.is_none() && n.tag != "placeholder" && n.tag != "text")
            .min_by_key(|(id, _)| id.0)
            .map(|(id, _)| id);

        if let Some(id) = root_id {
            self.root = Some(id);
            info!("activating root: [{}] (ElementId {})", self.nodes.get(id).unwrap().tag, id.0);
            self.activate(id, io);
        }
    }
}

// ── WriteMutations ──────────────────────────────────────────────

impl BehaviorTreeRenderer {
    /// Recursively create nodes for a template subtree.
    /// The root gets `id`; children get temporary IDs (>= 1_000_000).
    fn create_template_tree(
        &mut self,
        tnode: &TemplateNode,
        id: ElementId,
        parent: Option<ElementId>,
    ) {
        match tnode {
            TemplateNode::Element { tag, children, .. } => {
                let mut node = RendererNode::new(tag);
                node.parent = parent;
                self.nodes.insert(id, node);

                for child_tnode in *children {
                    let child_id = ElementId(self.next_temp_id);
                    self.next_temp_id += 1;
                    self.create_template_tree(child_tnode, child_id, Some(id));
                    if let Some(parent_node) = self.nodes.get_mut(id) {
                        parent_node.children.push(child_id);
                    }
                }
            }
            TemplateNode::Dynamic { .. } => {
                let mut node = RendererNode::new("placeholder");
                node.parent = parent;
                self.nodes.insert(id, node);
            }
            TemplateNode::Text { .. } => {
                let mut node = RendererNode::new("text");
                node.parent = parent;
                self.nodes.insert(id, node);
            }
        }
    }

    /// Remap a temp-ID node to a real ID, updating parent/child links.
    fn remap_node_id(&mut self, old_id: ElementId, new_id: ElementId) {
        if let Some(mut node) = self.nodes.remove(old_id) {
            // Update parent's children list
            if let Some(pid) = node.parent {
                if let Some(parent) = self.nodes.get_mut(pid) {
                    if let Some(pos) = parent.children.iter().position(|&c| c == old_id) {
                        parent.children[pos] = new_id;
                    }
                }
            }
            // Update children's parent refs
            for &cid in &node.children {
                if let Some(child) = self.nodes.get_mut(cid) {
                    child.parent = Some(new_id);
                }
            }
            self.nodes.insert(new_id, node);
        }
    }
}

impl WriteMutations for BehaviorTreeRenderer {
    fn load_template(&mut self, template: Template, index: usize, id: ElementId) {
        let tag = match &template.roots[index] {
            TemplateNode::Element { tag, .. } => *tag,
            _ => "?",
        };
        log::debug!("load_template: [{tag}] -> ElementId({}) (stack: {:?})", id.0, self.stack.iter().map(|e| e.0).collect::<Vec<_>>());
        self.create_template_tree(&template.roots[index], id, None);
        self.stack.push(id);
    }

    fn append_children(&mut self, id: ElementId, m: usize) {
        log::debug!("append_children: ElementId({}) += {} from stack {:?}", id.0, m, self.stack.iter().map(|e| e.0).collect::<Vec<_>>());
        let len = self.stack.len();
        let new_children: Vec<ElementId> = self.stack.drain(len - m..).collect();
        for &child_id in &new_children {
            if let Some(child) = self.nodes.get_mut(child_id) {
                child.parent = Some(id);
            }
        }
        if let Some(parent) = self.nodes.get_mut(id) {
            parent.children.extend(new_children);
        }
    }

    fn set_attribute(
        &mut self,
        name: &'static str,
        _ns: Option<&'static str>,
        value: &AttributeValue,
        id: ElementId,
    ) {
        if name == "args" {
            if let AttributeValue::Any(any) = value {
                if let Some(proto) = any.as_any().downcast_ref::<ProtoBytes>() {
                    if let Some(node) = self.nodes.get_mut(id) {
                        node.args_bytes = proto.0.clone();
                    }
                }
            }
        }
    }

    fn remove_node(&mut self, id: ElementId) {
        self.deactivate(id);
        if let Some(parent_id) = self.nodes.get(id).and_then(|n| n.parent) {
            if let Some(parent) = self.nodes.get_mut(parent_id) {
                parent.children.retain(|&c| c != id);
            }
        }
        if let Some(node) = self.nodes.remove(id) {
            for child_id in node.children {
                self.remove_node(child_id);
            }
        }
    }

    fn push_root(&mut self, id: ElementId) {
        self.stack.push(id);
    }

    fn assign_node_id(&mut self, path: &'static [u8], id: ElementId) {
        log::debug!("assign_node_id: path={:?} -> ElementId({}) (stack top: {:?})", path, id.0, self.stack.last().map(|e| e.0));
        let Some(&host_id) = self.stack.last() else { return };

        // Navigate path from host to find the temp-ID node
        let mut current = host_id;
        for &child_idx in path {
            let next = self.nodes.get(current)
                .and_then(|n| n.children.get(child_idx as usize).copied());
            match next {
                Some(cid) => current = cid,
                None => return,
            }
        }

        // Remap from temp ID to real ID
        if current != id {
            self.remap_node_id(current, id);
        }
    }

    fn create_placeholder(&mut self, id: ElementId) {
        self.nodes.insert(id, RendererNode::new("placeholder"));
        self.stack.push(id);
    }

    fn create_text_node(&mut self, _value: &str, id: ElementId) {
        self.nodes.insert(id, RendererNode::new("text"));
        self.stack.push(id);
    }

    fn replace_node_with(&mut self, id: ElementId, m: usize) {
        self.deactivate(id);
        let parent_id = self.nodes.get(id).and_then(|n| n.parent);
        let len = self.stack.len();
        let replacements: Vec<ElementId> = self.stack.drain(len - m..).collect();
        if let Some(pid) = parent_id {
            if let Some(parent) = self.nodes.get_mut(pid) {
                if let Some(pos) = parent.children.iter().position(|&c| c == id) {
                    parent.children.remove(pos);
                    for (i, &rep_id) in replacements.iter().enumerate() {
                        parent.children.insert(pos + i, rep_id);
                    }
                }
            }
            for &rep_id in &replacements {
                if let Some(node) = self.nodes.get_mut(rep_id) {
                    node.parent = parent_id;
                }
            }
        }
        self.nodes.remove(id);
    }

    fn replace_placeholder_with_nodes(&mut self, path: &'static [u8], m: usize) {
        // Pop m replacement nodes from the stack first
        let len = self.stack.len();
        let replacements: Vec<ElementId> = self.stack.drain(len - m..).collect();
        // Now the stack top is the host (the template root containing the placeholder)
        let Some(&host_id) = self.stack.last() else { return };
        log::debug!("replace_placeholder: host={} path={:?} replacements={:?}", host_id.0, path, replacements.iter().map(|e| e.0).collect::<Vec<_>>());

        // Walk path: each byte is a child index
        let mut target_id = host_id;
        for &child_idx in path {
            if let Some(node) = self.nodes.get(target_id) {
                if let Some(&cid) = node.children.get(child_idx as usize) {
                    target_id = cid;
                } else {
                    return;
                }
            } else {
                return;
            }
        }

        // target_id is the placeholder to replace
        let parent_id = self.nodes.get(target_id).and_then(|n| n.parent);
        if let Some(pid) = parent_id {
            if let Some(parent) = self.nodes.get_mut(pid) {
                if let Some(pos) = parent.children.iter().position(|&c| c == target_id) {
                    parent.children.remove(pos);
                    for (i, &rep_id) in replacements.iter().enumerate() {
                        parent.children.insert(pos + i, rep_id);
                    }
                }
            }
            for &rep_id in &replacements {
                if let Some(node) = self.nodes.get_mut(rep_id) {
                    node.parent = parent_id;
                }
            }
        }
        self.nodes.remove(target_id);
    }

    fn insert_nodes_after(&mut self, id: ElementId, m: usize) {
        let len = self.stack.len();
        let new_nodes: Vec<ElementId> = self.stack.drain(len - m..).collect();
        let parent_id = self.nodes.get(id).and_then(|n| n.parent);
        if let Some(pid) = parent_id {
            if let Some(parent) = self.nodes.get_mut(pid) {
                if let Some(pos) = parent.children.iter().position(|&c| c == id) {
                    for (i, &new_id) in new_nodes.iter().enumerate() {
                        parent.children.insert(pos + 1 + i, new_id);
                    }
                }
            }
            for &new_id in &new_nodes {
                if let Some(node) = self.nodes.get_mut(new_id) {
                    node.parent = parent_id;
                }
            }
        }
    }

    fn insert_nodes_before(&mut self, id: ElementId, m: usize) {
        let len = self.stack.len();
        let new_nodes: Vec<ElementId> = self.stack.drain(len - m..).collect();
        let parent_id = self.nodes.get(id).and_then(|n| n.parent);
        if let Some(pid) = parent_id {
            if let Some(parent) = self.nodes.get_mut(pid) {
                if let Some(pos) = parent.children.iter().position(|&c| c == id) {
                    for (i, &new_id) in new_nodes.iter().enumerate() {
                        parent.children.insert(pos + i, new_id);
                    }
                }
            }
            for &new_id in &new_nodes {
                if let Some(node) = self.nodes.get_mut(new_id) {
                    node.parent = parent_id;
                }
            }
        }
    }

    fn set_node_text(&mut self, _value: &str, _id: ElementId) {}
    fn create_event_listener(&mut self, _name: &'static str, _id: ElementId) {}
    fn remove_event_listener(&mut self, _name: &'static str, _id: ElementId) {}
}
