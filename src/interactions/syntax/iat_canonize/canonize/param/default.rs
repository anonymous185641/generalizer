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
use crate::interactions::syntax::iat_canonize::transformation::transfokind::InteractionTransformationKind;
use crate::interactions::syntax::iat_canonize::transformation::transfophase::InteractionTransformationPhase;
#[allow(dead_code)]
pub enum DefaultCanonizationProcess {
    Basic,
    FivePhases
}


impl DefaultCanonizationProcess {

    pub fn get_phases(&self) -> Vec<InteractionTransformationPhase> {
        match *self {
            DefaultCanonizationProcess::Basic => {
                vec![InteractionTransformationPhase::new(
                    vec![
                        InteractionTransformationKind::Simpl,
                        InteractionTransformationKind::LoopSimpl,
                        InteractionTransformationKind::FlushRight,
                        InteractionTransformationKind::InvertPar,
                        InteractionTransformationKind::InvertAlt,
                        InteractionTransformationKind::InvertTensor,
                        InteractionTransformationKind::AltDeduplicate,
                        InteractionTransformationKind::LoopSDeduplicate,
                    ]
                )]
            },
            DefaultCanonizationProcess::FivePhases => {
                let defactorize = vec![
                    InteractionTransformationKind::DeFactorizeLeft,
                    InteractionTransformationKind::DeFactorizeRight,
                    InteractionTransformationKind::AltDeduplicate,
                    InteractionTransformationKind::LoopSDeduplicate,
                ];
                let factorize_suff = vec![
                    InteractionTransformationKind::FactorizeSuffixSeq,
                    InteractionTransformationKind::FactorizeCommutativePar,
                    InteractionTransformationKind::AltDeduplicate,
                    InteractionTransformationKind::LoopSDeduplicate,
                ];
                let factorize_pref = vec![
                    InteractionTransformationKind::FactorizePrefixSeq,
                    InteractionTransformationKind::FactorizeCommutativePar,
                    InteractionTransformationKind::AltDeduplicate,
                    InteractionTransformationKind::LoopSDeduplicate,
                ];
                vec![
                    InteractionTransformationPhase::new(Self::simpl_phase()),
                    InteractionTransformationPhase::new(defactorize),
                    InteractionTransformationPhase::new(Self::simpl_phase()),
                    InteractionTransformationPhase::new(factorize_suff),
                    InteractionTransformationPhase::new(Self::simpl_phase()),
                    InteractionTransformationPhase::new(factorize_pref),
                    InteractionTransformationPhase::new(Self::simpl_phase())
                ]
            }
        }
    }


    fn simpl_phase() -> Vec<InteractionTransformationKind> {
        vec![
            InteractionTransformationKind::AltDeduplicate,
            InteractionTransformationKind::LoopSDeduplicate,
            InteractionTransformationKind::Simpl,
            InteractionTransformationKind::FlushRight,
            InteractionTransformationKind::InvertPar,
            InteractionTransformationKind::InvertAlt,
            InteractionTransformationKind::LoopSimpl,
            // ***
            //InteractionTransformationKind::StrictToSeq,
            //InteractionTransformationKind::ParToSeq,
            // ***
            //InteractionTransformationKind::SortActionContent
        ]
    }

}




