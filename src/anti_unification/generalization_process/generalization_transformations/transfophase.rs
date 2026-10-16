


use std::fmt::Formatter;
use crate::anti_unification::configuration::configuration::Configuration;
use crate::anti_unification::generalization_process::generalization_transformations::get_transfos::get_all_transfos::get_all_transformations_rec;
use crate::anti_unification::generalization_process::generalization_transformations::get_transfos::get_one_transfo::get_one_transformation_rec;
use crate::anti_unification::generalization_process::generalization_transformations::transfokind::ConfigurationTransformationKind;
use crate::anti_unification::generalization_process::generalization_transformations::transfores::ConfigurationTransformationResult;


pub struct ConfigurationTransformationPhase {
    pub transfos : Vec<ConfigurationTransformationKind>,
    /*
    pub get_all : bool,
    // a priority order in the application of the ConfigurationTransformationKind
    pub ordered : bool*/
}

impl std::fmt::Display for ConfigurationTransformationPhase {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut got = self.transfos.iter().fold("[".to_string(), |prev,phase|
            format!("{:},{:}",prev,phase)
        );
        got.push_str("]");
        write!(f, "{}", got)
    }
}

impl ConfigurationTransformationPhase {
    pub fn new(transfos : Vec<ConfigurationTransformationKind>,/*get_all : bool,ordered : bool*/) -> ConfigurationTransformationPhase {
        return ConfigurationTransformationPhase{transfos/*,get_all,ordered*/};
    }

    pub fn get_transfos(&self, configuration: &Configuration, get_all : bool) -> Vec<ConfigurationTransformationResult> {
        if get_all {
            return get_all_transformations_rec(&self.transfos, configuration);
        } else {
            match get_one_transformation_rec(&self.transfos, configuration) {
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


