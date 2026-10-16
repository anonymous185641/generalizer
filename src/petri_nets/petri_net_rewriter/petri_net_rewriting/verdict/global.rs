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
use graph_process_manager_core::manager::verdict::AbstractGlobalVerdict;
use crate::petri_nets::petri_net::petri_net::PetriNet;
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting::verdict::local::PetriNetRewritingLocalVerdict;
use crate::regex_inf::regex::regex::Regex;

#[derive(PartialEq, Eq, Debug)]
pub struct PetriNetRewritingGlobalVerdict {
    pub final_petri_nets: Vec<PetriNet<Regex<String>>>
}

impl fmt::Display for PetriNetRewritingGlobalVerdict {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"")
    }
}

impl AbstractGlobalVerdict<PetriNetRewritingLocalVerdict> for PetriNetRewritingGlobalVerdict {

    fn is_verdict_pertinent_for_process() -> bool {
        false
    }

    fn get_baseline_verdict() -> Self {
        PetriNetRewritingGlobalVerdict { final_petri_nets:vec![]}
    }

    fn update_with_local_verdict(self,
                                 local_verdict: &PetriNetRewritingLocalVerdict) -> Self {
        let mut ints = self.final_petri_nets;
        ints.push(local_verdict.got_petri_net.clone());
        Self{ final_petri_nets:ints}
    }

    fn is_goal_reached(&self, goal: &Option<Self>) -> bool {


        match goal.as_ref() {
            None => {
                false
            },
            Some( target_goal ) => {
                //self >= target_goal
                (self.final_petri_nets.len() > target_goal.final_petri_nets.len()) &&
                    match self.final_petri_nets.last() {
                        None => false,
                        Some(last_petri) =>{

                            last_petri.get_regex_from_final_petri_net().is_ok()
                        }
                    }
            }
        }
    }

    fn update_knowing_nodes_were_filtered_out(self,
                                              _has_filtered_nodes: bool) -> Self {
        self
    }

}


