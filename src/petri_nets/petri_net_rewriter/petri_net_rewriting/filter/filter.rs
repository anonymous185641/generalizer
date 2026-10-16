/*
Copyright 2020 Erwan Mahe (github.com/erwanM974)

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/



use std::fmt;

use graph_process_manager_core::handler::filter::AbstractFilter;
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting::filter::elim::PetriNetRewritingFilterEliminationKind;

pub struct PetriNetRewritingFilterCriterion {}

impl fmt::Display for PetriNetRewritingFilterCriterion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"")
    }
}

pub enum PetriNetRewritingFilter {}

impl fmt::Display for PetriNetRewritingFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"")
    }
}

impl AbstractFilter<PetriNetRewritingFilterCriterion, PetriNetRewritingFilterEliminationKind>  for PetriNetRewritingFilter {

    fn apply_filter(&self,
                    _depth: u32,
                    _node_counter: u32,
                    _criterion: &PetriNetRewritingFilterCriterion) -> Option<PetriNetRewritingFilterEliminationKind> {
        None
    }

}

