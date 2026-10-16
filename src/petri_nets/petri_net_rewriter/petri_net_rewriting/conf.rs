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



use graph_process_manager_core::manager::config::AbstractProcessConfiguration;
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting::context::PetriNetRewritingContext;
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting::filter::elim::PetriNetRewritingFilterEliminationKind;
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting::filter::filter::PetriNetRewritingFilterCriterion;
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting::handling::handler::PetriNetRewritingProcessHandler;
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting::node::PetriNetRewritingNodeKind;
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting::param::phase::PetriNetRewritingParameterization;
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting::priorities::PetriNetRewritingPriorities;
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting::step::PetriNetRewritingStepKind;
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting::verdict::global::PetriNetRewritingGlobalVerdict;
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting::verdict::local::PetriNetRewritingLocalVerdict;

pub struct PetriNetRewritingConfig {}

pub struct PetriNetRewritingStaticLocalVerdictAnalysisProof {}

impl AbstractProcessConfiguration for PetriNetRewritingConfig {
    type Context = PetriNetRewritingContext;
    type Parameterization = PetriNetRewritingParameterization;
    type NodeKind = PetriNetRewritingNodeKind;
    type StepKind = PetriNetRewritingStepKind;
    type Priorities = PetriNetRewritingPriorities;
    type FilterCriterion = PetriNetRewritingFilterCriterion;
    type FilterEliminationKind = PetriNetRewritingFilterEliminationKind;
    type LocalVerdict = PetriNetRewritingLocalVerdict;
    type StaticLocalVerdictAnalysisProof = PetriNetRewritingStaticLocalVerdictAnalysisProof;
    type GlobalVerdict = PetriNetRewritingGlobalVerdict;
    type ProcessHandler = PetriNetRewritingProcessHandler;
}
