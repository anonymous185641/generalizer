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
use crate::anti_unification::generalization_process::generalization_transformations::transfokind::ConfigurationTransformationKind;
use crate::anti_unification::generalization_process::generalization_transformations::transfophase::ConfigurationTransformationPhase;

#[allow(dead_code)]
pub enum DefaultGeneralizationProcess {
    Basic,
    ScPreserving,
    ScPreservingWithVariant
}


impl DefaultGeneralizationProcess {

    pub fn get_phases(&self) -> Vec<ConfigurationTransformationPhase> {
        match *self {
            DefaultGeneralizationProcess::Basic => {
                vec![ConfigurationTransformationPhase::new(
                    vec![
                        ConfigurationTransformationKind::Decompose,
                        ConfigurationTransformationKind::Solve,
                        ConfigurationTransformationKind::Recover,
                        ConfigurationTransformationKind::DecomposeA,
                        ConfigurationTransformationKind::DecomposeAC,
                        ConfigurationTransformationKind::DecomposeC,
                        ConfigurationTransformationKind::ExpandULeftDecompose,
                        ConfigurationTransformationKind::ExpandURightDecompose,
                        ConfigurationTransformationKind::ExpandUSameLeftDecompose,
                        ConfigurationTransformationKind::ExpandUSameRightDecompose,
                    ]
                )]
            },
            DefaultGeneralizationProcess::ScPreserving => {
                vec![ConfigurationTransformationPhase::new(
                    vec![
                        ConfigurationTransformationKind::Decompose,
                        ConfigurationTransformationKind::ScSolve,
                        ConfigurationTransformationKind::Recover,
                        //ConfigurationTransformationKind::Fail, // I remove FAIL here because it is checked elsewhere, in particular in the get_all_transformations_rec function
                        ConfigurationTransformationKind::DecomposeA,
                        ConfigurationTransformationKind::DecomposeAC,
                        ConfigurationTransformationKind::DecomposeC,
                        ConfigurationTransformationKind::DecomposeCSeq,
                        ConfigurationTransformationKind::DecomposeAC_Coreg,
                        ConfigurationTransformationKind::DecomposeA_Coreg,
                        ConfigurationTransformationKind::ExpandULeftDecompose,
                        ConfigurationTransformationKind::ExpandURightDecompose,
                        ConfigurationTransformationKind::ExpandUSameLeftDecompose,
                        ConfigurationTransformationKind::ExpandUSameRightDecompose,
                    ]
                )]
            },
            DefaultGeneralizationProcess::ScPreservingWithVariant => {
                vec![ConfigurationTransformationPhase::new(
                    vec![
                        ConfigurationTransformationKind::Variant,
                        ConfigurationTransformationKind::ScSolve,
                        ConfigurationTransformationKind::Recover,
                        ConfigurationTransformationKind::Decompose,
                        ConfigurationTransformationKind::DecomposeA,
                        ConfigurationTransformationKind::DecomposeAC,
                        ConfigurationTransformationKind::DecomposeC,
                        ConfigurationTransformationKind::DecomposeCSeq,
                        ConfigurationTransformationKind::DecomposeAC_Coreg,
                        ConfigurationTransformationKind::DecomposeA_Coreg,
                        ]
                )]
            }
        }
    }

}




