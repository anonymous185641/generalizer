



use std::fmt;
use graph_process_manager_core::delegate::priorities::AbstractPriorities;
use crate::anti_unification::generalization_process::generalization::step::GeneralizationStepKind;

pub struct GeneralizationPriorities {}

impl GeneralizationPriorities {

    pub fn new() -> GeneralizationPriorities {
        return GeneralizationPriorities {};
    }

    pub fn default() -> GeneralizationPriorities {
        GeneralizationPriorities::new()
    }
}

impl fmt::Display for GeneralizationPriorities {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"")
    }
}

impl AbstractPriorities<GeneralizationStepKind> for GeneralizationPriorities {
    fn get_priority_of_step(&self, _step: &GeneralizationStepKind) -> i32 {
        0
    }
}

