
use crate::anti_unification::configuration::configuration::Configuration;

use crate::anti_unification::error::ConfigurationError;
use crate::anti_unification::generalization_process::generalization_transformations::transfofunc::unit_element_rules::expand_same::expand_u_same_right::{can_apply_expand_u_same_right, expand_u_same_right};
use crate::anti_unification::generalization_process::generalization_transformations::transfofunc::unit_element_rules::generic_decompose::generic_decompose;

pub fn can_apply_expand_u_same_right_decompose(config : &Configuration) -> bool {
    can_apply_expand_u_same_right(config)
}

pub fn transfo_expand_u_same_right_decompose(config : &Configuration) -> Vec<Configuration> {

    let mut result_confs = Vec::new();

    //Expansions
    let expand_same_right_confs = expand_u_same_right(config);



    //And then decomposition
    for conf in expand_same_right_confs {

        result_confs.extend(generic_decompose(&conf));

    }

    //Return the resulting configurations
    result_confs
}

