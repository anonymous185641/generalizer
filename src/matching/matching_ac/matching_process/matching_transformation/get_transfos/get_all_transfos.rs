use crate::anti_unification::configuration::aut::AUT;
use crate::anti_unification::configuration::configuration::Configuration;
use crate::matching::matching_ac::matching_configuration::matching_configuration::MatchingConfiguration;
use crate::matching::matching_ac::matching_process::matching_transformation::transfokind::MatchingConfigurationTransformationKind;
use crate::matching::matching_ac::matching_process::matching_transformation::transfores::MatchingConfigurationTransformationResult;
use crate::terms::function::Function;
use crate::terms::substitution::substitution::Substitution;
use crate::terms::substitution::variable::Variable;
use crate::terms::term::Term;

pub fn get_all_transformations_rec(transfos : &Vec<MatchingConfigurationTransformationKind>,
                                   matching_configuration : &MatchingConfiguration) -> Vec<MatchingConfigurationTransformationResult> {
    if matching_configuration.unsolved_set.len() == 0 {
        return Vec::new();
    }
    let mut results : Vec<MatchingConfigurationTransformationResult> = Vec::new();

   for transfo_kind in transfos.iter() {
       // For each transformation kind, check if it is applicable to the current configuration, and if so, apply it.
       if transfo_kind.get_transformation_precondition()(matching_configuration) {
           let new_transfos : Vec<MatchingConfigurationTransformationResult> = transfo_kind.get_transformation()(matching_configuration)
               .into_iter().map(|x| MatchingConfigurationTransformationResult::new((*transfo_kind).clone(),x)).collect();
           results.extend(new_transfos);
       }
   }
   return results;
}






