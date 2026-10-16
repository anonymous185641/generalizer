


use std::fmt::Formatter;
use crate::matching::matching_ac::matching_configuration::matching_configuration::MatchingConfiguration;
use crate::matching::matching_ac::matching_process::matching_transformation::get_transfos::get_all_transfos::get_all_transformations_rec;
use crate::matching::matching_ac::matching_process::matching_transformation::get_transfos::get_one_transfo::get_one_transformation_rec;
use crate::matching::matching_ac::matching_process::matching_transformation::transfokind::MatchingConfigurationTransformationKind;
use crate::matching::matching_ac::matching_process::matching_transformation::transfores::MatchingConfigurationTransformationResult;

pub struct MatchingConfigurationTransformationPhase {
    pub transfos : Vec<MatchingConfigurationTransformationKind>,
    /*
    pub get_all : bool,
    // a priority order in the application of the ConfigurationTransformationKind
    pub ordered : bool*/
}

impl std::fmt::Display for MatchingConfigurationTransformationPhase {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut got = self.transfos.iter().fold("[".to_string(), |prev,phase|
            format!("{:},{:}",prev,phase)
        );
        got.push_str("]");
        write!(f, "{}", got)
    }
}

impl MatchingConfigurationTransformationPhase {
    pub fn new(transfos : Vec<MatchingConfigurationTransformationKind>, /*get_all : bool,ordered : bool*/) -> MatchingConfigurationTransformationPhase {
        return MatchingConfigurationTransformationPhase {transfos/*,get_all,ordered*/};
    }

    pub fn get_transfos(&self, matching_configuration: &MatchingConfiguration, get_all : bool) -> Vec<MatchingConfigurationTransformationResult> {
        if get_all {
            return get_all_transformations_rec(&self.transfos, matching_configuration);
        } else {
            match get_one_transformation_rec(&self.transfos, matching_configuration) {
                None => {
                    return Vec::new();
                },
                Some( got ) => {
                    return vec![got];
                }
            }
        }
    }

}


