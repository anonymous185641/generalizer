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


use graph_process_manager_core::delegate::node::GenericNode;
use graph_process_manager_core::handler::handler::AbstractProcessHandler;
use graph_process_manager_core::queued_steps::step::GenericStep;
use crate::anti_unification::generalization_process::generalization::conf::{GeneralizationConfig, GeneralizationStaticLocalVerdictAnalysisProof};
use crate::anti_unification::generalization_process::generalization::context::GeneralizationContext;
use crate::anti_unification::generalization_process::generalization::filter::filter::GeneralizationFilterCriterion;
use crate::anti_unification::generalization_process::generalization::node::GeneralizationNodeKind;
use crate::anti_unification::generalization_process::generalization::param::phase::GeneralizationParameterization;
use crate::anti_unification::generalization_process::generalization::step::GeneralizationStepKind;
use crate::anti_unification::generalization_process::generalization::verdict::local::GeneralizationLocalVerdict;



pub struct GeneralizationProcessHandler {}

impl AbstractProcessHandler<GeneralizationConfig> for GeneralizationProcessHandler {

    fn process_new_step(_context: &GeneralizationContext,
                        _param: &GeneralizationParameterization,
                        parent_state: &GenericNode<GeneralizationNodeKind>,
                        step_to_process: &GenericStep<GeneralizationStepKind>,
                        _new_state_id: u32,
                        _node_counter: u32) -> GeneralizationNodeKind {
        match step_to_process.kind {
            GeneralizationStepKind::GoToNextPhase => {
                GeneralizationNodeKind::new(parent_state.kind.configuration.clone(), parent_state.kind.phase + 1)
            },
            GeneralizationStepKind::Transform(ref result) => {
                GeneralizationNodeKind::new(result.result.clone(), parent_state.kind.phase)
            }
        }
    }

    fn get_criterion(_context: &GeneralizationContext,
                     _param: &GeneralizationParameterization,
                     _parent_state: &GenericNode<GeneralizationNodeKind>,
                     _step_to_process: &GenericStep<GeneralizationStepKind>,
                     _new_state_id: u32,
                     _node_counter: u32) -> GeneralizationFilterCriterion {
        GeneralizationFilterCriterion {}
    }

    fn collect_next_steps(_context: &GeneralizationContext,
                          param : &GeneralizationParameterization,
                          parent_node_kind: &GeneralizationNodeKind)
                          -> Vec<GeneralizationStepKind> {
        match param.phases.get(parent_node_kind.phase as usize) {
            None => {
                vec![]
            },
            Some(phase) => {
                let transfos = phase.get_transfos(&parent_node_kind.configuration,param.get_all);
                if transfos.is_empty() {
                    vec![GeneralizationStepKind::GoToNextPhase]
                } else {
                    transfos.into_iter()
                        .map(|r|
                            GeneralizationStepKind::Transform(r)
                        )
                        .collect()
                }
            }
        }
    }

    fn get_local_verdict_when_no_child(_context: &GeneralizationContext,
                                       _param : &GeneralizationParameterization,
                                       node_kind: &GeneralizationNodeKind) -> GeneralizationLocalVerdict {
        GeneralizationLocalVerdict {got_configuration:node_kind.configuration.clone()}
    }

    fn get_local_verdict_from_static_analysis(_context: &GeneralizationContext,
                                              _param : &GeneralizationParameterization,
                                              _node_kind: &mut GeneralizationNodeKind)
                                              -> Option<(GeneralizationLocalVerdict, GeneralizationStaticLocalVerdictAnalysisProof)> {
        None
    }

    fn pursue_process_after_static_verdict(_context: &GeneralizationContext,
                                           _param : &GeneralizationParameterization,
                                           _loc_verd: &GeneralizationLocalVerdict) -> bool {
        true
    }
}