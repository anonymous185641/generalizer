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

use crate::matching::matching_ac::matching_process::matching_transformation::transfokind::MatchingConfigurationTransformationKind;
use crate::matching::matching_ac::matching_process::matching_transformation::transfophase::MatchingConfigurationTransformationPhase;

#[allow(dead_code)]
pub enum DefaultMatchingProcess {
    Basic,
}


impl DefaultMatchingProcess {

    pub fn get_phases(&self) -> Vec<MatchingConfigurationTransformationPhase> {
        match *self {
            DefaultMatchingProcess::Basic => {
                vec![MatchingConfigurationTransformationPhase::new(
                    vec![
                        MatchingConfigurationTransformationKind::MDecompose,
                        MatchingConfigurationTransformationKind::MSolve,
                        MatchingConfigurationTransformationKind::Clash,
                    ]
                )]
            },
        }
    }

}




