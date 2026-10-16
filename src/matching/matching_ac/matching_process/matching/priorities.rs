



use std::fmt;
use graph_process_manager_core::delegate::priorities::AbstractPriorities;
use crate::matching::matching_ac::matching_process::matching::step::MatchingStepKind;

pub struct MatchingPriorities {}

impl MatchingPriorities {

    pub fn new() -> MatchingPriorities {
        return MatchingPriorities {};
    }

    pub fn default() -> MatchingPriorities {
        MatchingPriorities::new()
    }
}

impl fmt::Display for MatchingPriorities {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"")
    }
}

impl AbstractPriorities<MatchingStepKind> for MatchingPriorities {
    fn get_priority_of_step(&self, _step: &MatchingStepKind) -> i32 {
        0
    }
}

