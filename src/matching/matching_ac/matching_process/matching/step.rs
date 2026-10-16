use crate::matching::matching_ac::matching_process::matching_transformation::transfores::MatchingConfigurationTransformationResult;

pub enum MatchingStepKind {
    Transform(MatchingConfigurationTransformationResult),
    GoToNextPhase
}


