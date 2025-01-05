// fn main() {
//     println!("Hello, Bazel with Rust!");
// }

use tracing::{info, instrument, Level};

use tracing_subscriber::FmtSubscriber;

// #[async_trait]
pub trait Action<Spec, State, Result, SignalIn, SignalOut, Context, Estimate, AnySpec, AnyState, AnyResult> {
    async fn validate(
        &self,
        request: Spec,
    ) -> std::result::Result<AnySpec, Box<str>>;
}

struct NoOp {};

impl Action for NoOp {

}

struct SequenceActionSpec<'a> {
    children: &'a [ActionSpec<'a>],
}

enum ActionSpec<'a> {
    NoOp,
    Sequence { sequence: SequenceActionSpec<'a> },
}

// struct ActionSpec {
//     action_type: ActionType,
//     action_args:
//     a: str,
// }

#[instrument]
fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    let action_spec: ActionSpec = ActionSpec::Sequence {
        sequence: SequenceActionSpec {
            children: &[ActionSpec::NoOp {}],
        },
    };

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    info!(number_of_yaks = 3, "preparing to shave yaks");

    info!(all_yaks_shaved = true, "yak shaving completed.");
}
