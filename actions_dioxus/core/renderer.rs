//! BehaviorTreeRenderer -- the Dioxus custom renderer that executes
//! behavior trees.
//!
//! Implements `WriteMutations` to receive tree structure changes from
//! Dioxus, and provides `tick_actions()` to poll running nodes and
//! propagate results through the tree.
//!
//! All node types (composites and leaves alike) go through the same
//! unified `NodeBehavior` trait and `handle_response()` dispatch.

use std::collections::HashMap;

use dioxus_core::{AttributeValue, ElementId, Template, TemplateNode, WriteMutations};
use log::{info, warn};

use crate::actions::io::ActionIO;

use super::behavior::{ChildResult, NodeBehavior, NodeResponse};
use super::node::{NodeState, RendererNode};

// ── Node factory ────────────────────────────────────────────────

/// Creates a `NodeBehavior` for a core element from its tag and attributes.
///
/// Returns `None` for unknown tags (placeholders, text nodes, etc.).
fn create_node(
    tag: &str,
    attrs: &HashMap<&'static str, f64>,
) -> Option<Box<dyn NodeBehavior>> {
    match tag {
        "sequence" => Some(Box::new(
            crate::actions_dioxus::sequence::SequenceNode::from_attrs(attrs),
        )),
        "fallback" => Some(Box::new(
            crate::actions_dioxus::fallback::FallbackNode::from_attrs(attrs),
        )),
        "takeoff" => Some(Box::new(
            crate::actions_dioxus::takeoff::TakeoffNode::from_attrs(attrs),
        )),
        "land" => Some(Box::new(
            crate::actions_dioxus::land::LandNode::from_attrs(attrs),
        )),
        "goto" => Some(Box::new(
            crate::actions_dioxus::goto_waypoint::GotoWaypointNode::from_attrs(attrs),
        )),
        "home" => Some(Box::new(
            crate::actions_dioxus::return_home::ReturnHomeNode::from_attrs(attrs),
        )),
        "photo" => Some(Box::new(
            crate::actions_dioxus::take_photo::TakePhotoNode::from_attrs(attrs),
        )),
        "parallel" => Some(Box::new(
            crate::actions_dioxus::parallel::ParallelNode::from_attrs(attrs),
        )),
        _ => None,
    }
}

// ── Renderer ────────────────────────────────────────────────────

/// The Dioxus custom renderer for behavior trees.
///
/// Maintains a tree of `RendererNode`s that mirrors the Dioxus element
/// tree. Each active node holds a `Box<dyn NodeBehavior>` that drives
/// its lifecycle through a unified `handle_response()` dispatch.
///
/// # Lifecycle
///
/// 1. Dioxus calls `WriteMutations` methods to build/modify the tree.
/// 2. The renderer tracks parent-child relationships and element types.
/// 3. `activate()` creates a `NodeBehavior`, calls `on_activate`, and
///    dispatches the response (which may activate children, complete, etc.).
/// 4. `tick_actions()` calls `on_tick` on all active nodes each cycle.
/// 5. When a child completes, the parent's `on_child_complete` is called,
///    and its response is dispatched the same way.
pub struct BehaviorTreeRenderer {
    /// All nodes in the tree, keyed by Dioxus ElementId.
    nodes: HashMap<ElementId, RendererNode>,

    /// The Dioxus mutation stack. Mutations push/pop node IDs here.
    stack: Vec<ElementId>,

    /// Root element ID, if a tree is active.
    root: Option<ElementId>,

    /// Template tag cache: maps template pointer -> tag name.
    template_tags: HashMap<usize, &'static str>,

    /// Whether the mission completed (root propagated a result).
    mission_result: Option<ChildResult>,
}

impl BehaviorTreeRenderer {
    /// Create a new empty renderer.
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            stack: Vec::new(),
            root: None,
            template_tags: HashMap::new(),
            mission_result: None,
        }
    }

    /// Take the mission result if the root has completed.
    pub fn take_mission_result(&mut self) -> Option<ChildResult> {
        self.mission_result.take()
    }

    /// Number of tracked nodes.
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    // ── Unified response handling ───────────────────────────────

    /// Central dispatch: handle any `NodeResponse` from any node.
    ///
    /// All lifecycle methods (`on_activate`, `on_tick`, `on_child_complete`)
    /// funnel their responses through here.
    fn handle_response(&mut self, id: ElementId, response: NodeResponse, io: &ActionIO) {
        match response {
            NodeResponse::Running => {
                // Nothing to do -- node will be ticked next cycle.
            }
            NodeResponse::ActivateChild(index) => {
                let child_id = self.nodes.get(&id)
                    .and_then(|n| n.children.get(index).copied());
                if let Some(cid) = child_id {
                    self.activate(cid, io);
                } else {
                    warn!("ActivateChild({index}) out of bounds for node");
                }
            }
            NodeResponse::ActivateAllChildren => {
                let child_ids: Vec<ElementId> = self.nodes.get(&id)
                    .map(|n| n.children.clone())
                    .unwrap_or_default();
                for cid in child_ids {
                    self.activate(cid, io);
                }
            }
            NodeResponse::Success => {
                if let Some(node) = self.nodes.get_mut(&id) {
                    node.state = NodeState::Completed(ChildResult::Success);
                }
                self.propagate_up(id, ChildResult::Success, io);
            }
            NodeResponse::Failure => {
                if let Some(node) = self.nodes.get_mut(&id) {
                    node.state = NodeState::Completed(ChildResult::Failure);
                }
                self.propagate_up(id, ChildResult::Failure, io);
            }
        }
    }

    // ── Activation / deactivation ───────────────────────────────

    /// Activate a node: create its `NodeBehavior`, call `on_activate`,
    /// and dispatch the response.
    pub fn activate(&mut self, id: ElementId, io: &ActionIO) {
        let (tag, attrs) = match self.nodes.get(&id) {
            Some(n) => (n.tag, n.attrs.clone()),
            None => return,
        };

        if let Some(mut behavior) = create_node(tag, &attrs) {
            info!("[{tag}] activate");
            let response = behavior.on_activate(io);
            self.nodes.get_mut(&id).unwrap().state = NodeState::Active(behavior);
            self.handle_response(id, response, io);
        } else {
            warn!("[{tag}] unknown element -- cannot activate");
        }
    }

    /// Deactivate a node and all its descendants.
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

    /// Propagate a child's result to its parent.
    fn propagate_up(&mut self, child_id: ElementId, result: ChildResult, io: &ActionIO) {
        let parent_id = match self.nodes.get(&child_id).and_then(|n| n.parent) {
            Some(pid) => pid,
            None => {
                // Root completed -- mission done
                info!("mission {result:?}");
                self.mission_result = Some(result);
                return;
            }
        };

        // Find the child's index and the parent's child count
        let (child_index, child_count) = {
            let parent = match self.nodes.get(&parent_id) {
                Some(p) => p,
                None => return,
            };
            let idx = parent.children.iter().position(|&c| c == child_id).unwrap_or(0);
            (idx, parent.children.len())
        };

        // Call the parent's on_child_complete
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

    /// Poll all active nodes and propagate any completions.
    ///
    /// Call this once per tick cycle (~100ms).
    pub fn tick_actions(&mut self, io: &ActionIO) {
        // Collect responses from all active nodes
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

        // Dispatch responses (can't do this while iterating)
        for (id, response) in responses {
            self.handle_response(id, response, io);
        }
    }

    // ── Template helpers ────────────────────────────────────────

    /// Extract tag name from a template root node.
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

    /// Activate the root node after the initial `rebuild()`.
    ///
    /// Call this once after `dom.rebuild(&mut renderer)` to start
    /// execution of the behavior tree.
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

// ── WriteMutations implementation ───────────────────────────────

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
        if let Some(node) = self.nodes.get_mut(&id) {
            match value {
                AttributeValue::Float(v) => {
                    node.attrs.insert(name, *v);
                }
                AttributeValue::Int(v) => {
                    node.attrs.insert(name, *v as f64);
                }
                _ => {}
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
