use crate::anti_unification::configuration::aut::AUT;
use crate::anti_unification::configuration::configuration::Configuration;
use crate::anti_unification::generalization_process::generalization_transformations::transfofunc::fail::can_apply_fail;
use crate::anti_unification::generalization_process::generalization_transformations::transfokind::ConfigurationTransformationKind;
use crate::anti_unification::generalization_process::generalization_transformations::transfores::ConfigurationTransformationResult;
use crate::terms::function::Function;
use crate::terms::substitution::substitution::Substitution;
use crate::terms::substitution::variable::Variable;
use crate::terms::term::Term;

pub fn get_all_transformations_rec(transfos : &Vec<ConfigurationTransformationKind>,
                                   configuration : &Configuration) -> Vec<ConfigurationTransformationResult> {
    if configuration.active.len() == 0 {
        return Vec::new();
    }
    // Check FAIL HERE
    if can_apply_fail(configuration){
       // println!("FAIL");
        return Vec::new();
    }

    println!("Current configuration {}\n",configuration);

    let mut results : Vec<ConfigurationTransformationResult> = Vec::new();

   for transfo_kind in transfos.iter() {
       // For each transformation kind, check if it is applicable to the current configuration, and if so, apply it.
       if transfo_kind.get_transformation_precondition()(configuration) {
           println!("applying transformation {}\n",transfo_kind);
           //println!("head AUT {}\n",configuration.active[0].clone());
           let new_transfos : Vec<ConfigurationTransformationResult> = transfo_kind.get_transformation()(configuration)
               .into_iter().map(|x| ConfigurationTransformationResult::new((*transfo_kind).clone(),x)).collect();
           results.extend(new_transfos);
       }
   }
   return results;
}








