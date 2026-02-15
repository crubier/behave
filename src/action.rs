//! Core tick result type for behavior tree execution.

/// Result of a single tick of an action node.
#[derive(Debug, Clone)]
pub enum Tick<O, R> {
    /// Action is still running; here is the latest output.
    Running(O),
    /// Action completed successfully.
    Success(R),
    /// Action completed with failure.
    Failure(R),
}

impl<O, R> Tick<O, R> {
    pub fn is_running(&self) -> bool {
        matches!(self, Tick::Running(_))
    }
    pub fn is_done(&self) -> bool {
        !self.is_running()
    }
    pub fn is_success(&self) -> bool {
        matches!(self, Tick::Success(_))
    }
    pub fn is_failure(&self) -> bool {
        matches!(self, Tick::Failure(_))
    }
}
