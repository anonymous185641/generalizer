use std::collections::BTreeSet;
use std::fmt::Debug;
use std::hash::Hash;
use is_empty::IsEmpty;
use crate::petri_nets::petri_net_io::petri_net_parsing::petri_nets_string_parser::LiteralFromString;

pub trait AutomatonTransitionLabelBound:
Debug
+ Clone
+ PartialEq
+ Eq
+ Hash
+ PartialOrd
+ Ord
{
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AutomatonState{
    pub name: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AutomatonTransition<T>{
    pub label: T,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct  FiniteAutomaton<T: AutomatonTransitionLabelBound>{
    pub initial_state: AutomatonState,
    pub final_states: Vec<AutomatonState>,
    pub transitions: BTreeSet<(AutomatonState, AutomatonTransition<T>,AutomatonState)>,
    pub states: Vec<AutomatonState>,
}













