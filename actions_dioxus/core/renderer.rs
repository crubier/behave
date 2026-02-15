//! BehaviorTreeRenderer -- the Dioxus custom renderer that executes
//! behavior trees.
//!
//! Three data flows managed by the renderer:
//!
//! - **Output** (up): each tick, polls `behavior.output()` and stores
//!   it in the node's `Run` record.
//! - **Result** (up): on completion, calls `behavior.result()` and
//!   finalizes the `Run`. Child runs are attached to parent runs.
//! - **Command** (down): `send_command()` delivers data to a running node
//!   via `behavior.on_command()`.

use std::collections::HashMap;

use dioxus_core::{AttributeValue, ElementId, Template, TemplateNode, WriteMutations};
use log::{info, warn};

use crate::actions::io::ActionIO;

use super::behavior::{ChildResult, NodeBehavior, NodeResponse};
use super::data::{now_utime, ProtoBytes, Run, RunId, RunStatus};
use super::node::{NodeState, RendererNode};

// ── Node factory ────────────────────────────────────────────────

/// Create a `NodeBehavior` from a tag name and serialized proto Args.
fn create_node(tag: &str, args: &[u8]) -> Option<Box<dyn NodeBehavior>> {
    match tag {
        "sequence" => Some(Box::new(crate::actions_dioxus::sequence::SequenceNode::from_bytes(args))),
        "fallback" => Some(Box::new(crate::actions_dioxus::fallback::FallbackNode::from_bytes(args))),
        "takeoff"  => Some(Box::new(crate::actions_dioxus::takeoff::TakeoffNode::from_bytes(args))),
        "land"     => Some(Box::new(crate::actions_dioxus::land::LandNode::from_bytes(args))),
        "goto"     => Some(Box::new(crate::actions_dioxus::goto_waypoint::GotoWaypointNode::from_bytes(args))),
        "home"     => Some(Box::new(crate::actions_dioxus::return_home::ReturnHomeNode::from_bytes(args))),
        "photo"    => Some(Box::new(crate::actions_dioxus::take_photo::TakePhotoNode::from_bytes(args))),
        "parallel" => Some(Box::new(crate::actions_dioxus::parallel::ParallelNode::from_bytes(args))),
        _ => None,
    }
}

// ── Renderer ────────────────────────────────────────────────────

pub struct BehaviorTreeRenderer {
    /// All nodes, keyed by Dioxus ElementId.
    nodes: HashMap<ElementId, RendererNode>,

    /// Dioxus mutation stack.
    stack: Vec<ElementId>,

    /// Root element ID.
    root: Option<ElementId>,

    /// Template tag cache.
    template_tags: HashMap<usize, &'static str>,

    /// Completed mission run (set when root completes).
    mission_run: Option<Run>,

    /// Monotonic run ID counter.
    next_run_id: RunId,
}

impl BehaviorTreeRenderer {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            stack: Vec::new(),
            root: None,
            template_tags: HashMap::new(),
            mission_run: None,
            next_run_id: 1,
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
    /// Returns the root's Run with all children, including live output.
    pub fn snapshot(&self) -> Option<Run> {
        self.root.and_then(|id| self.build_run_snapshot(id))
    }

    /// Send a serialized protobuf command to a specific node.
    pub fn send_command(&mut self, id: ElementId, cmd: &[u8]) {
        if let Some(node) = self.nodes.get_mut(&id) {
            if let NodeState::Active(ref mut behavior) = node.state {
                behavior.on_command(cmd);
            }
        }
    }

    /// Send a serialized protobuf command to all active nodes with the given tag.
    pub fn send_command_by_tag(&mut self, tag: &str, cmd: &[u8]) {
        let ids: Vec<ElementId> = self
            .nodes
            .iter()
            .filter(|(_, n)| n.tag == tag && n.is_active())
            .map(|(&id, _)| id)
            .collect();
        for id in ids {
            self.send_command(id, cmd);
        }
    }

    /// Number of tracked nodes.
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    // ── Run snapshots ───────────────────────────────────────────

    /// Build a recursive run snapshot for the given node.
    fn build_run_snapshot(&self, id: ElementId) -> Option<Run> {
        let node = self.nodes.get(&id)?;
        let mut run = node.run.clone()?;

        // Recurse into children
        run.children.clear();
        for &child_id in &node.children {
            if let Some(child_run) = self.build_run_snapshot(child_id) {
                run.children.push(child_run);
            }
        }

        Some(run)
    }

    // ── Unified response handling ───────────────────────────────

    fn handle_response(&mut self, id: ElementId, response: NodeResponse, io: &ActionIO) {
        match response {
            NodeResponse::Running => {}
            NodeResponse::ActivateChild(index) => {
                let child_id = self
                    .nodes
                    .get(&id)
                    .and_then(|n| n.children.get(index).copied());
                if let Some(cid) = child_id {
                    self.activate(cid, io);
                } else {
                    warn!("ActivateChild({index}) out of bounds");
                }
            }
            NodeResponse::ActivateAllChildren => {
                let child_ids: Vec<ElementId> = self
                    .nodes
                    .get(&id)
                    .map(|n| n.children.clone())
                    .unwrap_or_default();
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
        let (tag, args) = match self.nodes.get(&id) {
            Some(n) => (n.tag, n.args_bytes.clone()),
            None => return,
        };

        if let Some(mut behavior) = create_node(tag, &args) {
            let run_id = self.alloc_run_id();
            let now = now_utime();
            info!("[{tag}] activate (run #{run_id})");

            let response = behavior.on_activate(io);

            let node = self.nodes.get_mut(&id).unwrap();
            node.start_run(run_id, now);
            node.state = NodeState::Active(behavior);

            self.handle_response(id, response, io);
        } else {
            warn!("[{tag}] unknown element");
        }
    }

    /// Finalize a node's Run: call `result_bytes()` + `state_bytes()`,
    /// set end time and status.
    fn complete_node(&mut self, id: ElementId, status: RunStatus) {
        let now = now_utime();

        // Collect child runs first (separate borrow scope)
        let child_runs: Vec<Run> = self
            .nodes
            .get(&id)
            .map(|n| {
                n.children
                    .iter()
                    .filter_map(|&cid| {
                        self.nodes
                            .get(&cid)
                            .and_then(|cn| cn.run.clone())
                    })
                    .collect()
            })
            .unwrap_or_default();

        // Now mutate the node
        if let Some(node) = self.nodes.get_mut(&id) {
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
        if let Some(node) = self.nodes.get(&id) {
            let children: Vec<_> = node.children.clone();
            for child_id in children {
                self.deactivate(child_id);
            }
        }
        if let Some(node) = self.nodes.get_mut(&id) {
            if node.is_active() {
                info!("[{}] deactivate", node.tag);
            }
            node.state = NodeState::Idle;
        }
    }

    // ── Result propagation ──────────────────────────────────────

    fn propagate_up(&mut self, child_id: ElementId, result: ChildResult, io: &ActionIO) {
        let parent_id = match self.nodes.get(&child_id).and_then(|n| n.parent) {
            Some(pid) => pid,
            None => {
                // Root completed -- capture mission run
                if let Some(node) = self.nodes.get(&child_id) {
                    if let Some(run) = &node.run {
                        info!("mission {:?} (run #{})", run.status, run.id);
                        self.mission_run = Some(run.clone());
                    }
                }
                return;
            }
        };

        let (child_index, child_count) = {
            let parent = match self.nodes.get(&parent_id) {
                Some(p) => p,
                None => return,
            };
            let idx = parent
                .children
                .iter()
                .position(|&c| c == child_id)
                .unwrap_or(0);
            (idx, parent.children.len())
        };

        let response = {
            let parent = match self.nodes.get_mut(&parent_id) {
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

    /// Poll all active nodes: update output, check for completions.
    pub fn tick_actions(&mut self, io: &ActionIO) {
        // 1. Update output + state for all active nodes
        for node in self.nodes.values_mut() {
            if let NodeState::Active(ref behavior) = node.state {
                if let Some(ref mut run) = node.run {
                    run.output = behavior.output_bytes();
                    run.state = behavior.state_bytes();
                }
            }
        }

        // 2. Collect tick responses
        let responses: Vec<(ElementId, NodeResponse)> = self
            .nodes
            .iter_mut()
            .filter_map(|(&id, node)| {
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

    // ── Template helpers ────────────────────────────────────────

    fn tag_from_template(&mut self, template: Template, index: usize) -> &'static str {
        let key = template.roots.as_ptr() as usize + index;
        if let Some(&tag) = self.template_tags.get(&key) {
            return tag;
        }
        let tag = match &template.roots[index] {
            TemplateNode::Element { tag, .. } => *tag,
            _ => "unknown",
        };
        self.template_tags.insert(key, tag);
        tag
    }

    /// Activate the root after `dom.rebuild()`.
    pub fn activate_root(&mut self, io: &ActionIO) {
        let root_id = self
            .nodes
            .iter()
            .find(|(_, n)| n.parent.is_none() && n.tag != "placeholder" && n.tag != "text")
            .map(|(&id, _)| id);

        if let Some(id) = root_id {
            self.root = Some(id);
            self.activate(id, io);
        }
    }
}

// ── WriteMutations ──────────────────────────────────────────────

impl WriteMutations for BehaviorTreeRenderer {
    fn load_template(&mut self, template: Template, index: usize, id: ElementId) {
        let tag = self.tag_from_template(template, index);
        self.nodes.insert(id, RendererNode::new(tag));
        self.stack.push(id);
    }

    fn append_children(&mut self, id: ElementId, m: usize) {
        let len = self.stack.len();
        let new_children: Vec<ElementId> = self.stack.drain(len - m..).collect();
        for &child_id in &new_children {
            if let Some(child) = self.nodes.get_mut(&child_id) {
                child.parent = Some(id);
            }
        }
        if let Some(parent) = self.nodes.get_mut(&id) {
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
                    if let Some(node) = self.nodes.get_mut(&id) {
                        node.args_bytes = proto.0.clone();
                    }
                }
            }
        }
    }

    fn remove_node(&mut self, id: ElementId) {
        self.deactivate(id);
        if let Some(parent_id) = self.nodes.get(&id).and_then(|n| n.parent) {
            if let Some(parent) = self.nodes.get_mut(&parent_id) {
                parent.children.retain(|&c| c != id);
            }
        }
        if let Some(node) = self.nodes.remove(&id) {
            for child_id in node.children {
                self.remove_node(child_id);
            }
        }
    }

    fn push_root(&mut self, id: ElementId) {
        self.stack.push(id);
    }

    fn assign_node_id(&mut self, _path: &'static [u8], _id: ElementId) {}

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
        let parent_id = self.nodes.get(&id).and_then(|n| n.parent);
        let len = self.stack.len();
        let replacements: Vec<ElementId> = self.stack.drain(len - m..).collect();
        if let Some(pid) = parent_id {
            if let Some(parent) = self.nodes.get_mut(&pid) {
                if let Some(pos) = parent.children.iter().position(|&c| c == id) {
                    parent.children.remove(pos);
                    for (i, &rep_id) in replacements.iter().enumerate() {
                        parent.children.insert(pos + i, rep_id);
                    }
                }
            }
            for &rep_id in &replacements {
                if let Some(node) = self.nodes.get_mut(&rep_id) {
                    node.parent = parent_id;
                }
            }
        }
        self.nodes.remove(&id);
    }

    fn replace_placeholder_with_nodes(&mut self, _path: &'static [u8], _m: usize) {}

    fn insert_nodes_after(&mut self, id: ElementId, m: usize) {
        let len = self.stack.len();
        let new_nodes: Vec<ElementId> = self.stack.drain(len - m..).collect();
        let parent_id = self.nodes.get(&id).and_then(|n| n.parent);
        if let Some(pid) = parent_id {
            if let Some(parent) = self.nodes.get_mut(&pid) {
                if let Some(pos) = parent.children.iter().position(|&c| c == id) {
                    for (i, &new_id) in new_nodes.iter().enumerate() {
                        parent.children.insert(pos + 1 + i, new_id);
                    }
                }
            }
            for &new_id in &new_nodes {
                if let Some(node) = self.nodes.get_mut(&new_id) {
                    node.parent = parent_id;
                }
            }
        }
    }

    fn insert_nodes_before(&mut self, id: ElementId, m: usize) {
        let len = self.stack.len();
        let new_nodes: Vec<ElementId> = self.stack.drain(len - m..).collect();
        let parent_id = self.nodes.get(&id).and_then(|n| n.parent);
        if let Some(pid) = parent_id {
            if let Some(parent) = self.nodes.get_mut(&pid) {
                if let Some(pos) = parent.children.iter().position(|&c| c == id) {
                    for (i, &new_id) in new_nodes.iter().enumerate() {
                        parent.children.insert(pos + i, new_id);
                    }
                }
            }
            for &new_id in &new_nodes {
                if let Some(node) = self.nodes.get_mut(&new_id) {
                    node.parent = parent_id;
                }
            }
        }
    }

    fn set_node_text(&mut self, _value: &str, _id: ElementId) {}
    fn create_event_listener(&mut self, _name: &'static str, _id: ElementId) {}
    fn remove_event_listener(&mut self, _name: &'static str, _id: ElementId) {}
}
