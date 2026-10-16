
use crate::anti_unification::configuration::configuration::Configuration;
use crate::anti_unification::generalization_process::generalization_transformations::transfokind::ConfigurationTransformationKind;

pub struct ConfigurationTransformationResult {
    pub kind : ConfigurationTransformationKind,
    pub result : Configuration
}

impl ConfigurationTransformationResult {
    pub fn new(kind : ConfigurationTransformationKind,
               result : Configuration) -> ConfigurationTransformationResult {
        return ConfigurationTransformationResult{kind,result};
    }
}