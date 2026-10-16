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




use std::hash::{Hash, Hasher};

use graph_process_manager_core::manager::config::AbstractNodeKind;
use crate::petri_nets::petri_net::petri_net::PetriNet;
use crate::regex_inf::regex::regex::Regex;

#[derive(Clone, Debug)]
pub struct PetriNetRewritingNodeKind {
    pub petri_net: PetriNet<Regex<String>>,
    pub phase : u32
}

impl PetriNetRewritingNodeKind {
    pub fn new(petri_net: PetriNet<Regex<String>>,
               phase: u32) -> Self {
        PetriNetRewritingNodeKind { petri_net, phase }
    }
}

impl PartialEq for PetriNetRewritingNodeKind {
    fn eq(&self, other: &Self) -> bool {
        self.petri_net == other.petri_net
            && self.phase == other.phase
    }
}

impl Eq for PetriNetRewritingNodeKind {}

impl Hash for PetriNetRewritingNodeKind {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.petri_net.hash(state);
        self.phase.hash(state);
    }
}
impl AbstractNodeKind for PetriNetRewritingNodeKind {
    fn is_included_for_memoization(&self, memoized_node: &Self) -> bool {
        self.petri_net == memoized_node.petri_net && self.phase == memoized_node.phase
    }
}

