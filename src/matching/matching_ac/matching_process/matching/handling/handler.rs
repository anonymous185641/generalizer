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
use crate::matching::matching_ac::matching_process::matching::conf::{MatchingConfig, MatchingStaticLocalVerdictAnalysisProof};
use crate::matching::matching_ac::matching_process::matching::context::MatchingContext;
use crate::matching::matching_ac::matching_process::matching::filter::filter::MatchingFilterCriterion;
use crate::matching::matching_ac::matching_process::matching::node::MatchingNodeKind;
use crate::matching::matching_ac::matching_process::matching::param::phase::MatchingParameterization;
use crate::matching::matching_ac::matching_process::matching::step::MatchingStepKind;
use crate::matching::matching_ac::matching_process::matching::verdict::local::MatchingLocalVerdict;

pub struct MatchingProcessHandler {}

impl AbstractProcessHandler<MatchingConfig> for MatchingProcessHandler {

    fn process_new_step(_context: &MatchingContext,
                        _param: &MatchingParameterization,
                        parent_state: &GenericNode<MatchingNodeKind>,
                        step_to_process: &GenericStep<MatchingStepKind>,
                        _new_state_id: u32,
                        _node_counter: u32) -> MatchingNodeKind {
        match step_to_process.kind {
            MatchingStepKind::GoToNextPhase => {
                MatchingNodeKind::new(parent_state.kind.configuration.clone(), parent_state.kind.phase + 1)
            },
            MatchingStepKind::Transform(ref result) => {
                MatchingNodeKind::new(result.result.clone(), parent_state.kind.phase)
            }
        }
    }

    fn get_criterion(_context: &MatchingContext,
                     _param: &MatchingParameterization,
                     _parent_state: &GenericNode<MatchingNodeKind>,
                     _step_to_process: &GenericStep<MatchingStepKind>,
                     _new_state_id: u32,
                     _node_counter: u32) -> MatchingFilterCriterion {
        MatchingFilterCriterion {}
    }

    fn collect_next_steps(_context: &MatchingContext,
                          param : &MatchingParameterization,
                          parent_node_kind: &MatchingNodeKind)
                          -> Vec<MatchingStepKind> {
        match param.phases.get(parent_node_kind.phase as usize) {
            None => {
                vec![]
            },
            Some(phase) => {
                let transfos = phase.get_transfos(&parent_node_kind.configuration,param.get_all);
                if transfos.is_empty() {
                    vec![MatchingStepKind::GoToNextPhase]
                } else {
                    transfos.into_iter()
                        .map(|r|
                            MatchingStepKind::Transform(r)
                        )
                        .collect()
                }
            }
        }
    }

    fn get_local_verdict_when_no_child(_context: &MatchingContext,
                                       _param : &MatchingParameterization,
                                       node_kind: &MatchingNodeKind) -> MatchingLocalVerdict {
        MatchingLocalVerdict {got_configuration:node_kind.configuration.clone()}
    }

    fn get_local_verdict_from_static_analysis(_context: &MatchingContext,
                                              _param : &MatchingParameterization,
                                              _node_kind: &mut MatchingNodeKind)
                                              -> Option<(MatchingLocalVerdict, MatchingStaticLocalVerdictAnalysisProof)> {
        None
    }

    fn pursue_process_after_static_verdict(_context: &MatchingContext,
                                           _param : &MatchingParameterization,
                                           _loc_verd: &MatchingLocalVerdict) -> bool {
        true
    }
}