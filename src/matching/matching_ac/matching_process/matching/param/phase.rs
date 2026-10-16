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
use crate::matching::matching_ac::matching_process::matching::param::default::DefaultMatchingProcess;
use crate::matching::matching_ac::matching_process::matching_transformation::transfophase::MatchingConfigurationTransformationPhase;

pub struct MatchingParameterization {
    pub phases : Vec<MatchingConfigurationTransformationPhase>,
    pub get_all : bool
}

impl MatchingParameterization {
    #[allow(dead_code)]
    pub fn new(phases: Vec<MatchingConfigurationTransformationPhase>, get_all: bool) -> Self {
        Self { phases, get_all }
    }

    pub fn from_default(def : DefaultMatchingProcess, get_all: bool) -> Self {
        Self{ phases : def.get_phases(), get_all}
    }
}

impl AbstractProcessParameterization for MatchingParameterization {
    fn get_param_as_strings(&self) -> Vec<String> {
        let mut strs = vec!["process = Generalization".to_string()];
        strs.push(format!("get_all_transformations = {:}", self.get_all));
        for (x,phase) in self.phases.iter().enumerate() {
            strs.push(format!("phase {:} = {:}", x+1, phase));
        }
        strs
    }
}