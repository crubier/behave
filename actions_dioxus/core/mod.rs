//! Core behavior tree renderer infrastructure.
//!
//! - [`behavior`] -- `NodeBehavior` trait (unified for all node types)
//! - [`elements`] -- Custom Dioxus element definitions for `rsx!`
//! - [`node`] -- `RendererNode` types (the "real DOM")
//! - [`renderer`] -- `BehaviorTreeRenderer` implementing `WriteMutations`

pub mod behavior;
pub mod elements;
pub mod node;
pub mod renderer;

pub use behavior::{ChildResult, NodeBehavior, NodeResponse};
pub use renderer::BehaviorTreeRenderer;
