#![allow(dead_code)]
use std::sync::Arc;

use rust_fsm::*;
use tokio::sync::Mutex;
#[cfg(feature = "tracing")]
use tracing::trace;

use super::experiment::ExperimentError;

state_machine! {
    derive(Debug)

    pub(crate) ExperimentState(Init)

    Init(Started) => Running,
    Running(Entered) => Tests,
    Closed(Failed) => Termination,
    Tests => {
        WaitClose => Pass,
        Failed => Closed,
    }
}

pub(crate) async fn consume_mu(
    this: Arc<Mutex<StateMachine<ExperimentState>>>,
    state: ExperimentStateInput,
) -> Result<(), ExperimentError> {
    let mut this = this.lock_owned().await;
    #[cfg(feature = "tracing")]
    trace!(name: "experiment-state", "before: {:#?} -> maybe: {:#?}", this.state(), state);
    this.consume(&state)?;
    #[cfg(feature = "tracing")]
    trace!(name: "experiment-state", "after: {:#?}", this.state());
    drop(this);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ok_path() {
        let mut state: StateMachine<ExperimentState> = StateMachine::new();
        state.consume(&ExperimentStateInput::Started).unwrap();
        state.consume(&ExperimentStateInput::Entered).unwrap();
        state.consume(&ExperimentStateInput::WaitClose).unwrap();
    }

    #[test]
    fn test_fail_path() {
        let mut state: StateMachine<ExperimentState> = StateMachine::new();
        state.consume(&ExperimentStateInput::Started).unwrap();
        state.consume(&ExperimentStateInput::Entered).unwrap();
        state.consume(&ExperimentStateInput::Failed).unwrap();
    }

    #[test]
    fn test_invalid_path_fail_without_runtime() {
        let mut state: StateMachine<ExperimentState> = StateMachine::new();
        state.consume(&ExperimentStateInput::Started).unwrap();
        state
            .consume(&ExperimentStateInput::Failed)
            .expect_err("must fail");
    }

    #[test]
    fn test_invalid_path_no_experiment_reuse() {
        let mut state: StateMachine<ExperimentState> = StateMachine::new();
        state.consume(&ExperimentStateInput::Started).unwrap();
        state.consume(&ExperimentStateInput::Entered).unwrap();
        state.consume(&ExperimentStateInput::WaitClose).unwrap();
        state
            .consume(&ExperimentStateInput::Started)
            .expect_err("must fail");
    }
}
