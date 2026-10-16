


use std::fmt::Formatter;
use crate::petri_nets::petri_net::petri_net::PetriNet;
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting_transformations::get_transfos::get_all_transfos::get_all_transformations_rec;
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting_transformations::get_transfos::get_one_transfo::get_one_transformation_rec;
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting_transformations::transfokind::PetriNetTransformationKind;
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting_transformations::transfores::PetriNetTransformationResult;
use crate::regex_inf::regex::regex::Regex;

pub struct PetriNetTransformationPhase {
    pub transfos : Vec<PetriNetTransformationKind>,
    /*
    pub get_all : bool,
    // a priority order in the application of the PetriNetTransformationKind
    pub ordered : bool*/
}

impl std::fmt::Display for PetriNetTransformationPhase {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut got = self.transfos.iter().fold("[".to_string(), |prev,phase|
            format!("{:},{:}",prev,phase)
        );
        got.push_str("]");
        write!(f, "{}", got)
    }
}

impl PetriNetTransformationPhase {
    pub fn new(transfos : Vec<PetriNetTransformationKind>,/*get_all : bool,ordered : bool*/) -> PetriNetTransformationPhase {
        return PetriNetTransformationPhase {transfos/*,get_all,ordered*/};
    }

    pub fn get_transfos(&self, petri_net: &PetriNet<Regex<String>>, get_all : bool) -> Vec<PetriNetTransformationResult> {
        if get_all {
            return get_all_transformations_rec(&self.transfos, petri_net);
        } else {
            match get_one_transformation_rec(&self.transfos, petri_net) {
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


