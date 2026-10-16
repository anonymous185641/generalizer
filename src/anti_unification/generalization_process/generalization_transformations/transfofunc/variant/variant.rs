use std::fmt::Display;
use crate::terms::substitution::substitution::Substitution;
use crate::terms::term::Term;

#[derive(Clone, PartialEq, Debug, Eq,Hash,Ord,PartialOrd)]
pub struct Variant{
    pub term:Term,
    pub substitution: Substitution
}


impl Variant {
    pub fn new(term:Term,substitution: Substitution) -> Self {
        Self { term, substitution }
    }
}

impl Display for Variant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "t={}\nsigma={}", self.term, self.substitution)
    }   
}









