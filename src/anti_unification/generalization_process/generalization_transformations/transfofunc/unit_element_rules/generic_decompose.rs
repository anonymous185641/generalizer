use crate::anti_unification::configuration::configuration::Configuration;
use crate::anti_unification::generalization_process::generalization_transformations::transfofunc::decompose::{can_apply_decompose, transfo_decompose};
use crate::anti_unification::generalization_process::generalization_transformations::transfofunc::decompose_a::{can_apply_decompose_a, transfo_decompose_a};
use crate::anti_unification::generalization_process::generalization_transformations::transfofunc::decompose_ac::{can_apply_decompose_ac, transfo_decompose_ac};
use crate::anti_unification::generalization_process::generalization_transformations::transfofunc::decompose_c::{can_apply_decompose_c, transfo_decompose_c};

pub fn generic_decompose(config : &Configuration) ->Vec<Configuration>{

        if can_apply_decompose(config){
            transfo_decompose(config)
        }
        else if can_apply_decompose_a(config){
            transfo_decompose_a(config)
        }
        else if can_apply_decompose_c(config){
            transfo_decompose_c(config)
        }
        else if can_apply_decompose_ac(config){
            transfo_decompose_ac(config)
        }

        else{
            vec![]
        }

    }


