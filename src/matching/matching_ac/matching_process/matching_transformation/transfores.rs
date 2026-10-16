
use crate::anti_unification::configuration::configuration::Configuration;
use crate::matching::matching_ac::matching_configuration::matching_configuration::MatchingConfiguration;
use crate::matching::matching_ac::matching_process::matching_transformation::transfokind::MatchingConfigurationTransformationKind;

pub struct MatchingConfigurationTransformationResult {
    pub kind : MatchingConfigurationTransformationKind,
    pub result : MatchingConfiguration
}

impl MatchingConfigurationTransformationResult {
    pub fn new(kind : MatchingConfigurationTransformationKind,
               result : MatchingConfiguration) -> MatchingConfigurationTransformationResult {
        return MatchingConfigurationTransformationResult {kind,result};
    }
}