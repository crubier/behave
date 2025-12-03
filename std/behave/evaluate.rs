use async_trait::async_trait;

/// The Evaluator trait provides offline evaluation capabilities for behaviors.
/// 
/// This trait enables behavior evaluation in various contexts:
/// - Offline validation and testing
/// - Cloud-based evaluation pipelines
/// - Local development environments
/// 
/// # Use Cases
/// 
/// - Pre-execution validation of behavior arguments
/// - Resource usage estimation
/// - Cost prediction
/// - Behavior composition through argument expansion
/// 
/// # Implementation Notes
/// 
/// Implementations should be stateless and side-effect free to enable:
/// - Parallel execution
/// - Caching
/// - Distribution across compute resources
#[async_trait]
pub trait Evaluator<BehaviorArgument, BehaviorResult, BehaviorMetrics> {
    /// Validates a behavior argument before execution.
    /// 
    /// This method performs static analysis of the argument to ensure:
    /// - All required fields are present
    /// - Values are within expected ranges
    /// - Dependencies are available
    /// - Resource requirements can be met
    async fn validate(&self, argument: &BehaviorArgument) -> Result<(), String>;

    /// Estimates behavior execution metrics without running the behavior.
    /// 
    /// Provides predictions for:
    /// - Resource usage (CPU, memory, network, etc.)
    /// - Execution time
    /// - Cost
    /// - Success probability
    async fn estimate(&self, argument: &BehaviorArgument) -> Result<BehaveMetrics, String>;

    /// Expands a behavior argument into a more detailed form.
    /// 
    /// Used for:
    /// - Breaking down complex behaviors
    /// - Resolving references and dependencies
    /// - Applying default values
    /// - Normalizing input format
    async fn expand(&self, argument: &BehaviorArgument) -> Result<BehaviorArgument, String>;
} 