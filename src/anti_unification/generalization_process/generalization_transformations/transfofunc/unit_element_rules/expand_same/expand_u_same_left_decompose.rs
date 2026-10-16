
use crate::anti_unification::configuration::configuration::Configuration;
use crate::anti_unification::generalization_process::generalization_transformations::transfofunc::unit_element_rules::expand_same::expand_u_same_left::{can_apply_expand_u_same_left, expand_u_same_left};
use crate::anti_unification::generalization_process::generalization_transformations::transfofunc::unit_element_rules::generic_decompose::generic_decompose;

pub fn can_apply_expand_u_same_left_decompose(config : &Configuration) -> bool {
    can_apply_expand_u_same_left(config)
}

pub fn transfo_expand_u_same_left_decompose(config : &Configuration) -> Vec<Configuration> {

    let mut result_confs = Vec::new();

    //Expansions
    let expand_same_left_confs = expand_u_same_left(config);
    
    //And then decomposition
    for conf in expand_same_left_confs {
        result_confs.extend(generic_decompose(&conf));
    }

    //Return the resulting configurations
    result_confs
    
}

