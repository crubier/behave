use async_trait::async_trait;

/// The Runner trait provides runtime execution capabilities for behaviors.
/// 
/// This trait enables behavior execution on agents in various contexts:
/// - Local agent execution
/// - Remote robot control
/// - Cloud-based processing
/// - Edge device operations
/// 
/// # Use Cases
/// 
/// - Real-time behavior execution
/// - Stateful operation management
/// - Input/output streaming
/// - Resource lifecycle control
/// 
/// # Implementation Notes
/// 
/// Implementations manage stateful execution and should:
/// - Handle resource allocation/deallocation
/// - Maintain execution state
/// - Support interruption and resumption
/// - Manage real-time constraints
#[async_trait]
pub trait Runner<BehaveArgument, BehaveResult, BehaveStatus, BehaveState, BehaveInput, BehaveOutput> {
    /// Starts or continues the behavior execution.
    /// 
    /// This method:
    /// - Initializes required resources
    /// - Sets up execution environment
    /// - Begins the main execution loop
    /// - Returns results upon completion
    async fn run(&mut self, argument: BehaveArgument) -> Result<BehaveResult, String>;
    
    /// Temporarily stops the behavior execution.
    /// 
    /// Used to:
    /// - Pause long-running operations
    /// - Save current state
    /// - Free up resources temporarily
    /// - Enable later resumption
    async fn suspend(&mut self) -> Result<BehaveState, String>;
    
    /// Resumes the behavior from a suspended state.
    /// 
    /// Handles:
    /// - State restoration
    /// - Resource reacquisition
    /// - Execution continuation
    /// - Context recovery
    async fn resume(&mut self, argument: BehaveArgument, state: BehaveState) -> Result<BehaveResult, String>;
    
    /// Permanently stops the behavior execution.
    /// 
    /// Ensures:
    /// - Clean resource cleanup
    /// - Final state recording
    /// - Proper shutdown sequence
    /// - Status reporting
    async fn cancel(&mut self) -> Result<BehaveStatus, String>;

    /// Processes incoming data during execution.
    /// 
    /// Enables:
    /// - Real-time input handling
    /// - Stream processing
    /// - Dynamic behavior adjustment
    /// - Progress feedback
    async fn receive(&mut self, input: BehaveInput) -> Result<BehaveOutput, String>;
} 