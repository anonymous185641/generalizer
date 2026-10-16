
use crate::petri_nets::petri_net::petri_net::PetriNet;
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting_transformations::transfokind::PetriNetTransformationKind;
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting_transformations::transfores::PetriNetTransformationResult;
use crate::regex_inf::regex::regex::Regex;
use crate::terms::term::Term;

pub fn get_one_transformation_rec(transfos : &Vec<PetriNetTransformationKind>,
                                  configuration : &PetriNet<Regex<String>>) -> Option<PetriNetTransformationResult> {

    todo!()
}

/*
fn get_one_transformation_inner(transfos : &Vec<PetriNetTransformationKind>,
                                configuration : &PetriNet) -> Option<PetriNetTransformationResult> {
    for transfo_kind in transfos {
        let mut new_transfos : Vec<PetriNetTransformationResult> = transfo_kind.get_transformation()(configuration)
            .into_iter().map(|x| PetriNetTransformationResult::new((*transfo_kind).clone(),x)).collect();
        if new_transfos.len() > 0 {
            return Some(new_transfos.remove(0));
        }
    }
    return None;
}

 */

