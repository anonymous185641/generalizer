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
use crate::interactions::syntax::iat_exploration::explo::context::{ExplorationContext, ExplorationParameterization};
use crate::interactions::syntax::iat_exploration::explo::filter::elim::ExplorationFilterEliminationKind;
use crate::interactions::syntax::iat_exploration::explo::filter::filter::ExplorationFilterCriterion;
use crate::interactions::syntax::iat_exploration::explo::handling::handler::ExplorationProcessHandler;
use crate::interactions::syntax::iat_exploration::explo::node::ExplorationNodeKind;
use crate::interactions::syntax::iat_exploration::explo::priorities::ExplorationPriorities;
use crate::interactions::syntax::iat_exploration::explo::step::ExplorationStepKind;
use crate::interactions::syntax::iat_exploration::explo::verdict::global::ExplorationGlobalVerdict;
use crate::interactions::syntax::iat_exploration::explo::verdict::local::ExplorationLocalVerdict;


pub struct ExplorationConfig {}

pub struct ExplorationStaticLocalVerdictAnalysisProof{}

impl AbstractProcessConfiguration for ExplorationConfig {
    type Context = ExplorationContext;
    type Parameterization = ExplorationParameterization;
    type NodeKind = ExplorationNodeKind;
    type StepKind = ExplorationStepKind;
    type Priorities = ExplorationPriorities;
    type FilterCriterion = ExplorationFilterCriterion;
    type FilterEliminationKind = ExplorationFilterEliminationKind;
    type LocalVerdict = ExplorationLocalVerdict;
    type StaticLocalVerdictAnalysisProof = ExplorationStaticLocalVerdictAnalysisProof;
    type GlobalVerdict = ExplorationGlobalVerdict;
    type ProcessHandler = ExplorationProcessHandler;
}
