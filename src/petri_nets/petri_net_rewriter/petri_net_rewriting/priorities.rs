



use std::fmt;
use graph_process_manager_core::delegate::priorities::AbstractPriorities;
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting::step::PetriNetRewritingStepKind;

pub struct PetriNetRewritingPriorities {}

impl PetriNetRewritingPriorities {

    pub fn new() -> PetriNetRewritingPriorities {
        return PetriNetRewritingPriorities {};
    }

    pub fn default() -> PetriNetRewritingPriorities {
        PetriNetRewritingPriorities::new()
    }
}

impl fmt::Display for PetriNetRewritingPriorities {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"")
    }
}

impl AbstractPriorities<PetriNetRewritingStepKind> for PetriNetRewritingPriorities {
    fn get_priority_of_step(&self, _step: &PetriNetRewritingStepKind) -> i32 {
        0
    }
}

