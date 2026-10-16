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
use crate::matching::matching_ac::matching_configuration::matching_configuration::MatchingConfiguration;
use crate::matching::matching_ac::matching_process::matching::verdict::local::MatchingLocalVerdict;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct MatchingGlobalVerdict {
    pub final_configurations: Vec<MatchingConfiguration>
}

impl fmt::Display for MatchingGlobalVerdict {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"")
    }
}

impl AbstractGlobalVerdict<MatchingLocalVerdict> for MatchingGlobalVerdict {

    fn is_verdict_pertinent_for_process() -> bool {
        false
    }

    fn get_baseline_verdict() -> Self {
        MatchingGlobalVerdict { final_configurations:vec![]}
    }

    fn update_with_local_verdict(self,
                                 local_verdict: &MatchingLocalVerdict) -> Self {
        let mut confs = self.final_configurations;
        confs.push(local_verdict.got_configuration.clone());
        Self{ final_configurations:confs}
    }

    fn is_goal_reached(&self, goal: &Option<Self>) -> bool {
        match goal.as_ref() {
            None => {
                false
            },
            Some( target_goal ) => {
                //self >= target_goal
                (self.final_configurations.len() > target_goal.final_configurations.len()) && 
                    match self.final_configurations.last() {
                        None => false,
                        Some(last_config) =>last_config.unsolved_set.is_empty()
                    }
            }
        }
    }

    fn update_knowing_nodes_were_filtered_out(self,
                                              _has_filtered_nodes: bool) -> Self {
        self
    }

}


