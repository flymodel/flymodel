#![allow(dead_code)]
use rust_fsm::*;

state_machine! {
    derive(Debug)

    pub(crate) ExperimentState(Init)

    Init(Started) => Transition,
    Running(WaitClose) => Transition,
    Closed(Failed) => Progressing [AttemptExperiment],
    Transition => {
        Started => Running,
        WaitClose => Closed,
        Failed => Closed,
    }
}
