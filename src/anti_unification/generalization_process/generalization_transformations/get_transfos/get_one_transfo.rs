use crate::anti_unification::configuration::configuration::Configuration;
use crate::anti_unification::generalization_process::generalization_transformations::transfokind::ConfigurationTransformationKind;
use crate::anti_unification::generalization_process::generalization_transformations::transfores::ConfigurationTransformationResult;
use crate::terms::term::Term;

pub fn get_one_transformation_rec(transfos : &Vec<ConfigurationTransformationKind>,
                                  configuration : &Configuration) -> Option<ConfigurationTransformationResult> {

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

