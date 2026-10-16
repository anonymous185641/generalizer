
use std::fmt::Formatter;
use strum_macros::IntoStaticStr;
use crate::petri_nets::petri_net::petri_net::PetriNet;
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting_transformations::transfofunc::dpo_rules::alt_loop::{apply_alternative_loop, can_apply_alternative_loop};
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting_transformations::transfofunc::dpo_rules::alt_preparation::{apply_alternative_preparation, can_apply_alternative_preparation};
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting_transformations::transfofunc::dpo_rules::alt_preparation2::{apply_alternative_preparation2, can_apply_alternative_preparation2};
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting_transformations::transfofunc::dpo_rules::alternative::{apply_alternative, can_apply_alternative};
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting_transformations::transfofunc::dpo_rules::concatenation::{apply_concatenation, can_apply_concatenation};
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting_transformations::transfofunc::dpo_rules::concatenation_loop::{apply_concatenation_loop, can_apply_concatenation_loop};
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting_transformations::transfofunc::dpo_rules::int_preparation::{apply_interleaving_preparation, can_apply_interleaving_preparation};
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting_transformations::transfofunc::dpo_rules::int_preparation2::{apply_interleaving_preparation2, can_apply_interleaving_preparation2};
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting_transformations::transfofunc::choice_interleaving_separation::r1::{apply_r1, can_apply_r1};
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting_transformations::transfofunc::choice_interleaving_separation::r2::{apply_r2, can_apply_r2};
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting_transformations::transfofunc::dpo_rules::interleaving::{apply_interleaving, can_apply_interleaving};
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting_transformations::transfofunc::dpo_rules::kleene1::{apply_kleene1, can_apply_kleene1};
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting_transformations::transfofunc::dpo_rules::kleene2::{apply_kleene2, can_apply_kleene2};
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting_transformations::transfofunc::dpo_rules::kleene_preparation::{apply_kleene_preparation, can_apply_kleene_preparation};
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting_transformations::transfofunc::dpo_rules::loop0::{apply_loop0, can_apply_loop0};
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting_transformations::transfofunc::dpo_rules::loop1::{apply_loop1, can_apply_loop1};
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting_transformations::transfofunc::dpo_rules::loop2::{apply_loop2, can_apply_loop2};
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting_transformations::transfofunc::dpo_rules::loop3::{apply_loop3, can_apply_loop3};
use crate::regex_inf::regex::regex::Regex;

#[derive(IntoStaticStr,Clone, PartialEq, Debug, Eq, Hash)]
pub enum PetriNetTransformationKind {
    R1,
    R2,
    Concatenation,
    Alternative,
    Interleaving,
    Loop0,
}

impl std::fmt::Display for PetriNetTransformationKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let as_static_str : &'static str = self.into();
        write!(f,"{}", as_static_str)
    }
}



impl PetriNetTransformationKind {
    pub fn to_string(&self) -> String {
        let as_static_str : &'static str = self.into();
        return as_static_str.to_string();
    }

    pub fn get_transformation_precondition(&self)-> fn(&PetriNet<Regex<String>>) -> bool {
        match self{
            PetriNetTransformationKind::R1 => {
                return can_apply_r1;
            },
            PetriNetTransformationKind::R2=>{
                return can_apply_r2;
            },
            PetriNetTransformationKind::Concatenation => {
                return can_apply_concatenation;
            },
            PetriNetTransformationKind::Alternative => {
                return can_apply_alternative;
            },
            PetriNetTransformationKind::Interleaving => {
                return can_apply_interleaving;
            },
            PetriNetTransformationKind::Loop0=>{
                return can_apply_loop0;
            },
        }
    }
    pub fn get_transformation(&self) -> fn(&PetriNet<Regex<String>>) -> Vec<PetriNet<Regex<String>>> {
        match self {
            PetriNetTransformationKind::R1 => {
                return apply_r1;
            },
           PetriNetTransformationKind::R2 => {
                return apply_r2;
            },
            PetriNetTransformationKind::Concatenation => {
                return apply_concatenation;
            },
            PetriNetTransformationKind::Alternative => {
                return apply_alternative;
            },
            PetriNetTransformationKind::Interleaving => {
                return apply_interleaving;
            },
            PetriNetTransformationKind::Loop0 => {
                return apply_loop0;
            }

        }
    }
}

