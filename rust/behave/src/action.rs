use std::result::Result;

#[async_trait]
pub trait Action<Spec, State, Result, SignalIn, SignalOut, Context,Estimate, AnySpec,AnyState, AnyResult> {
  async fn validate(&self,request: Spec, ) -> Result<AnySpec, Box<str>>;
}
