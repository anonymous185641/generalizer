
use crate::petri_nets::petri_net::petri_net::PetriNet;
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting_transformations::transfokind::PetriNetTransformationKind;
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting_transformations::transfores::PetriNetTransformationResult;
use crate::regex_inf::regex::regex::Regex;

pub fn get_all_transformations_rec(transfos : &Vec<PetriNetTransformationKind>,
                                   petri_net: &PetriNet<Regex<String>>) -> Vec<PetriNetTransformationResult> {
    
    
    let mut results : Vec<PetriNetTransformationResult> = Vec::new();

   for transfo_kind in transfos.iter() {
       // For each transformation kind, check if it is applicable to the current configuration, and if so, apply it.
       if transfo_kind.get_transformation_precondition()(petri_net) {
           let new_transfos : Vec<PetriNetTransformationResult> = transfo_kind.get_transformation()(petri_net)
               .into_iter().map(|x| PetriNetTransformationResult::new((*transfo_kind).clone(),x)).collect();
           results.extend(new_transfos);
       }
   }
   return results;
     
    
}




/*
fn get_all_transformations_inner(transfos : &Vec<PetriNetTransformationKind>,
                                 configuration: &PetriNet) -> Vec<PetriNetTransformationResult> {

    let mut results : Vec<PetriNetTransformationResult> = Vec::new();
    for transfo_kind in transfos {
        let new_transfos : Vec<PetriNetTransformationResult> = transfo_kind.get_transformation()(configuration)
            .into_iter().map(|x| PetriNetTransformationResult::new((*transfo_kind).clone(),x)).collect();
        results.extend(new_transfos);
    }
    return results;
}
 */



