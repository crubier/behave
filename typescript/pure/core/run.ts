

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

export interface Runner<BehaveArgument, BehaveResult, BehaveStatus, BehaveState, BehaveInput, BehaveOutput> {
  /// Starts or continues the behavior execution.
  /// 
  /// This method:
  /// - Initializes required resources
  /// - Sets up execution environment
  /// - Begins the main execution loop
  /// - Returns results upon completion
  run(argument: BehaveArgument): Promise<BehaveResult>;

  /// Temporarily stops the behavior execution.
  /// 
  /// Used to:
  /// - Pause long-running operations
  /// - Save current state
  /// - Free up resources temporarily
  /// - Enable later resumption
  suspend(): Promise<BehaveState>;

  /// Resumes the behavior from a suspended state.
  /// 
  /// Handles:
  /// - State restoration
  /// - Resource reacquisition
  /// - Execution continuation
  /// - Context recovery
  resume(argument: BehaveArgument, state: BehaveState): Promise<BehaveResult>;

  /// Permanently stops the behavior execution.
  /// 
  /// Ensures:
  /// - Clean resource cleanup
  /// - Final state recording
  /// - Proper shutdown sequence
  /// - Status reporting
  cancel(): Promise<BehaveStatus>;

  /// Processes incoming data during execution.
  /// 
  /// Enables:
  /// - Real-time input handling
  /// - Stream processing
  /// - Dynamic behavior adjustment
  /// - Progress feedback
  receive(input: BehaveInput): Promise<BehaveOutput>;
} 