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




use std::hash::Hash;

use graph_process_manager_core::manager::config::AbstractNodeKind;
use crate::anti_unification::configuration::configuration::Configuration;

#[derive(Clone, PartialEq, Debug, Eq, Hash)]
pub struct GeneralizationNodeKind {
    pub configuration: Configuration,
    pub phase : u32
}

impl GeneralizationNodeKind {
    pub fn new(configuration: Configuration,
               phase: u32) -> Self {
        GeneralizationNodeKind { configuration, phase }
    }
}


impl AbstractNodeKind for GeneralizationNodeKind {
    fn is_included_for_memoization(&self, memoized_node: &Self) -> bool {
        self.configuration == memoized_node.configuration && self.phase == memoized_node.phase
    }
}

