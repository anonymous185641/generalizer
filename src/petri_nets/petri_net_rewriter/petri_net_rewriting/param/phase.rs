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


use graph_process_manager_core::manager::config::AbstractProcessParameterization;
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting::param::default::DefaultPetriNetRewritingProcess;
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting_transformations::transfophase::PetriNetTransformationPhase;

pub struct PetriNetRewritingParameterization {
    pub phases : Vec<PetriNetTransformationPhase>,
    pub get_all : bool
}

impl PetriNetRewritingParameterization {
    #[allow(dead_code)]
    pub fn new(phases: Vec<PetriNetTransformationPhase>, get_all: bool) -> Self {
        Self { phases, get_all }
    }

    pub fn from_default(def : DefaultPetriNetRewritingProcess, get_all: bool) -> Self {
        Self{ phases : def.get_phases(), get_all}
    }
}

impl AbstractProcessParameterization for PetriNetRewritingParameterization {
    fn get_param_as_strings(&self) -> Vec<String> {
        let mut strs = vec!["process = PetriNetRewriting".to_string()];
        strs.push(format!("get_all_transformations = {:}", self.get_all));
        for (x,phase) in self.phases.iter().enumerate() {
            strs.push(format!("phase {:} = {:}", x+1, phase));
        }
        strs
    }
}