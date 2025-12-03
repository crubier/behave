use behave_std::behaviors::print::v1::*;
use behave_std::behave::Evaluator;
use async_trait::async_trait;

pub struct PrintEvaluator {
    // Add fields here as needed
}

#[async_trait]
impl Evaluator<PrintArgument, PrintResult, PrintMetrics> for PrintEvaluator {
    async fn validate(&self, argument: &PrintArgument) -> Result<(), String> {
        println!("Validating print behavior argument");
        Ok(())
    }

    async fn estimate(&self, argument: &PrintArgument) -> Result<PrintMetrics, String> {
        println!("Estimating print behavior metrics");
        Ok(PrintMetrics::default())
    }

    async fn expand(&self, argument: &PrintArgument) -> Result<PrintArgument, String> {
        println!("Expanding print behavior argument");
        Ok(vec![argument.clone()])
    }

}

#[tokio::main]
pub async fn main() {
    let evaluator = PrintEvaluator {};
    let argument = PrintArgument::default();

    if let Err(e) = evaluator.validate(&argument).await {
        eprintln!("Error validating argument: {}", e);
        return;
    }

    if let Err(e) = evaluator.evaluate(&argument, &PrintResult::default()).await {
        eprintln!("Error evaluating behavior: {}", e);
    }
}
