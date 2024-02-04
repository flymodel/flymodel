#![allow(dead_code)]
use std::fmt::Display;

use rust_fsm::*;

state_machine! {
    derive(Debug)

    pub(crate) ExperimentState(Init)

    Init(Started) => Running,
    Running(WaitClose) => Transition,
    Closed(Failed) => Termination,
    Transition => {
        Started => Running,
        WaitClose => Pass,
        Failed => Fail,
    }
}

impl Display for ExperimentStateState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_string().fmt(f)
    }
}
