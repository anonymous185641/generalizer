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
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting::conf::{PetriNetRewritingConfig, PetriNetRewritingStaticLocalVerdictAnalysisProof};
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting::context::PetriNetRewritingContext;
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting::filter::filter::PetriNetRewritingFilterCriterion;
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting::node::PetriNetRewritingNodeKind;
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting::param::phase::PetriNetRewritingParameterization;
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting::step::PetriNetRewritingStepKind;
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting::verdict::local::PetriNetRewritingLocalVerdict;

pub struct PetriNetRewritingProcessHandler {}

impl AbstractProcessHandler<PetriNetRewritingConfig> for PetriNetRewritingProcessHandler {

    fn process_new_step(_context: &PetriNetRewritingContext,
                        _param: &PetriNetRewritingParameterization,
                        parent_state: &GenericNode<PetriNetRewritingNodeKind>,
                        step_to_process: &GenericStep<PetriNetRewritingStepKind>,
                        _new_state_id: u32,
                        _node_counter: u32) -> PetriNetRewritingNodeKind {
        match step_to_process.kind {
            PetriNetRewritingStepKind::GoToNextPhase => {
                PetriNetRewritingNodeKind::new(parent_state.kind.petri_net.clone(), parent_state.kind.phase + 1)
            },
            PetriNetRewritingStepKind::Transform(ref result) => {
                PetriNetRewritingNodeKind::new(result.result.clone(), parent_state.kind.phase)
            }
        }
    }

    fn get_criterion(_context: &PetriNetRewritingContext,
                     _param: &PetriNetRewritingParameterization,
                     _parent_state: &GenericNode<PetriNetRewritingNodeKind>,
                     _step_to_process: &GenericStep<PetriNetRewritingStepKind>,
                     _new_state_id: u32,
                     _node_counter: u32) -> PetriNetRewritingFilterCriterion {
        PetriNetRewritingFilterCriterion {}
    }

    fn collect_next_steps(_context: &PetriNetRewritingContext,
                          param : &PetriNetRewritingParameterization,
                          parent_node_kind: &PetriNetRewritingNodeKind)
                          -> Vec<PetriNetRewritingStepKind> {
        match param.phases.get(parent_node_kind.phase as usize) {
            None => {
                vec![]
            },
            Some(phase) => {
                let transfos = phase.get_transfos(&parent_node_kind.petri_net,param.get_all);
                if transfos.is_empty() {
                    vec![PetriNetRewritingStepKind::GoToNextPhase]
                } else {
                    transfos.into_iter()
                        .map(|r|
                            PetriNetRewritingStepKind::Transform(r)
                        )
                        .collect()
                }
            }
        }
    }

    fn get_local_verdict_when_no_child(_context: &PetriNetRewritingContext,
                                       _param : &PetriNetRewritingParameterization,
                                       node_kind: &PetriNetRewritingNodeKind) -> PetriNetRewritingLocalVerdict {
        PetriNetRewritingLocalVerdict {got_petri_net:node_kind.petri_net.clone()}
    }

    fn get_local_verdict_from_static_analysis(_context: &PetriNetRewritingContext,
                                              _param : &PetriNetRewritingParameterization,
                                              _node_kind: &mut PetriNetRewritingNodeKind)
                                              -> Option<(PetriNetRewritingLocalVerdict, PetriNetRewritingStaticLocalVerdictAnalysisProof)> {
        None
    }

    fn pursue_process_after_static_verdict(_context: &PetriNetRewritingContext,
                                           _param : &PetriNetRewritingParameterization,
                                           _loc_verd: &PetriNetRewritingLocalVerdict) -> bool {
        true
    }
}