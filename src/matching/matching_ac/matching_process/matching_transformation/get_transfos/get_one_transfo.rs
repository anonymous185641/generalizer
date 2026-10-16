use crate::anti_unification::configuration::configuration::Configuration;
use crate::matching::matching_ac::matching_configuration::matching_configuration::MatchingConfiguration;
use crate::matching::matching_ac::matching_process::matching_transformation::transfokind::MatchingConfigurationTransformationKind;
use crate::matching::matching_ac::matching_process::matching_transformation::transfores::MatchingConfigurationTransformationResult;
use crate::terms::term::Term;

pub fn get_one_transformation_rec(transfos : &Vec<MatchingConfigurationTransformationKind>,
                                  configuration : &MatchingConfiguration) -> Option<MatchingConfigurationTransformationResult> {

    todo!()
}

/*
fn get_one_transformation_inner(transfos : &Vec<ConfigurationTransformationKind>,
                                configuration : &Configuration) -> Option<ConfigurationTransformationResult> {
    for transfo_kind in transfos {
        let mut new_transfos : Vec<ConfigurationTransformationResult> = transfo_kind.get_transformation()(configuration)
            .into_iter().map(|x| ConfigurationTransformationResult::new((*transfo_kind).clone(),x)).collect();
        if new_transfos.len() > 0 {
            return Some(new_transfos.remove(0));
        }
    }
    return None;
}

 */

