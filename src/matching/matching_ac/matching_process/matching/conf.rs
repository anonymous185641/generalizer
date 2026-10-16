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
use crate::matching::matching_ac::matching_process::matching::context::MatchingContext;
use crate::matching::matching_ac::matching_process::matching::filter::elim::MatchingFilterEliminationKind;
use crate::matching::matching_ac::matching_process::matching::filter::filter::MatchingFilterCriterion;
use crate::matching::matching_ac::matching_process::matching::handling::handler::MatchingProcessHandler;
use crate::matching::matching_ac::matching_process::matching::node::MatchingNodeKind;
use crate::matching::matching_ac::matching_process::matching::param::phase::MatchingParameterization;
use crate::matching::matching_ac::matching_process::matching::priorities::MatchingPriorities;
use crate::matching::matching_ac::matching_process::matching::step::MatchingStepKind;
use crate::matching::matching_ac::matching_process::matching::verdict::global::MatchingGlobalVerdict;
use crate::matching::matching_ac::matching_process::matching::verdict::local::MatchingLocalVerdict;

pub struct MatchingConfig {}

pub struct MatchingStaticLocalVerdictAnalysisProof {}

impl AbstractProcessConfiguration for MatchingConfig {
    type Context = MatchingContext;
    type Parameterization = MatchingParameterization;
    type NodeKind = MatchingNodeKind;
    type StepKind = MatchingStepKind;
    type Priorities = MatchingPriorities;
    type FilterCriterion = MatchingFilterCriterion;
    type FilterEliminationKind = MatchingFilterEliminationKind;
    type LocalVerdict = MatchingLocalVerdict;
    type StaticLocalVerdictAnalysisProof = MatchingStaticLocalVerdictAnalysisProof;
    type GlobalVerdict = MatchingGlobalVerdict;
    type ProcessHandler = MatchingProcessHandler;
}
