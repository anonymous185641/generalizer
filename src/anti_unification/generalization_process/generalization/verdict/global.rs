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
use crate::anti_unification::configuration::configuration::Configuration;
use crate::anti_unification::generalization_process::generalization::verdict::local::GeneralizationLocalVerdict;


#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct GeneralizationGlobalVerdict {
    pub final_configurations: Vec<Configuration>
}

impl fmt::Display for GeneralizationGlobalVerdict {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"")
    }
}

impl AbstractGlobalVerdict<GeneralizationLocalVerdict> for GeneralizationGlobalVerdict {

    fn is_verdict_pertinent_for_process() -> bool {
        false
    }

    fn get_baseline_verdict() -> Self {
        GeneralizationGlobalVerdict { final_configurations:vec![]}
    }

    fn update_with_local_verdict(self,
                                 local_verdict: &GeneralizationLocalVerdict) -> Self {
        let mut ints = self.final_configurations;
        ints.push(local_verdict.got_configuration.clone());
        Self{ final_configurations:ints}
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
                        Some(last_config) =>last_config.active.is_empty()
                    }
            }
        }
    }

    fn update_knowing_nodes_were_filtered_out(self,
                                              _has_filtered_nodes: bool) -> Self {
        self
    }

}


