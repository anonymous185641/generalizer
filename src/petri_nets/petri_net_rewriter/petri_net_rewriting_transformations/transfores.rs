
use crate::petri_nets::petri_net::petri_net::PetriNet;
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting_transformations::transfokind::PetriNetTransformationKind;
use crate::regex_inf::regex::regex::Regex;

pub struct PetriNetTransformationResult {
    pub kind : PetriNetTransformationKind,
    pub result : PetriNet<Regex<String>>
}

impl PetriNetTransformationResult {
    pub fn new(kind : PetriNetTransformationKind,
               result : PetriNet<Regex<String>>) -> PetriNetTransformationResult {
        return PetriNetTransformationResult {kind,result};
    }
}