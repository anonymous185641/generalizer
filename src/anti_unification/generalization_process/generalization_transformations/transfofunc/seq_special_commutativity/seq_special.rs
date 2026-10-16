use crate::anti_unification::configuration::aut::AUT;
use crate::anti_unification::configuration::configuration::Configuration;
use crate::terms::term::Term;
use crate::utils::misc::have_common_element;

impl AUT{

    pub fn seq_special_commutativity_applicable(&self) -> bool {
        if self.t1.is_head_seq() && self.t2.is_head_seq() && !have_common_element(&self.t1.lifelines_ids(), &self.t2.lifelines_ids()){
            true
        }
        else{
            false
        }
    }
}


impl Term{
    pub fn is_head_seq(&self) -> bool {
        match self{
            Term::Variable(_) => false,
            Term::Function(f) => {
                if f.signature.name == "seq"{
                    true
                }
                else{
                    false
                }
            }
        }
    }
}