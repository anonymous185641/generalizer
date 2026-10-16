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
use crate::anti_unification::generalization_process::generalization::context::GeneralizationContext;
use crate::anti_unification::generalization_process::generalization::filter::elim::GeneralizationFilterEliminationKind;
use crate::anti_unification::generalization_process::generalization::filter::filter::GeneralizationFilterCriterion;
use crate::anti_unification::generalization_process::generalization::handling::handler::GeneralizationProcessHandler;
use crate::anti_unification::generalization_process::generalization::node::GeneralizationNodeKind;
use crate::anti_unification::generalization_process::generalization::param::phase::GeneralizationParameterization;
use crate::anti_unification::generalization_process::generalization::priorities::GeneralizationPriorities;
use crate::anti_unification::generalization_process::generalization::step::GeneralizationStepKind;
use crate::anti_unification::generalization_process::generalization::verdict::global::GeneralizationGlobalVerdict;
use crate::anti_unification::generalization_process::generalization::verdict::local::GeneralizationLocalVerdict;



pub struct GeneralizationConfig {}

pub struct GeneralizationStaticLocalVerdictAnalysisProof {}

impl AbstractProcessConfiguration for GeneralizationConfig {
    type Context = GeneralizationContext;
    type Parameterization = GeneralizationParameterization;
    type NodeKind = GeneralizationNodeKind;
    type StepKind = GeneralizationStepKind;
    type Priorities = GeneralizationPriorities;
    type FilterCriterion = GeneralizationFilterCriterion;
    type FilterEliminationKind = GeneralizationFilterEliminationKind;
    type LocalVerdict = GeneralizationLocalVerdict;
    type StaticLocalVerdictAnalysisProof = GeneralizationStaticLocalVerdictAnalysisProof;
    type GlobalVerdict = GeneralizationGlobalVerdict;
    type ProcessHandler = GeneralizationProcessHandler;
}
