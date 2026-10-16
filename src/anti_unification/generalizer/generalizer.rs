use std::fmt;
use crate::terms::substitution::substitution::Substitution;
use crate::terms::term::Term;

#[derive(Clone, Debug)]
pub struct Generalizer {
    pub t: Term,
    pub sub1: Substitution,
    pub sub2: Substitution,
}

impl Generalizer {
    #[allow(dead_code)]
    pub fn new(t: &Term, sub1: &Substitution, sub2: &Substitution) -> Self {
        Self{t: t.clone(), sub1: sub1.clone(), sub2: sub2.clone()}
    }

}

impl fmt::Display for Generalizer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        write!(f, "{} \n σ₁ = {} \n σ₂ = {}",self.t,self.sub1,self.sub2)
        /*
        if let Some(history) = &self.history {
            write!(f, "{} \n σ₁ = {} \n σ₂ = {} \n {}",self.t,self.sub1,self.sub2,history )
        }
        else{
            write!(f, "{} \n σ₁ = {} \n σ₂ = {}",self.t,self.sub1,self.sub2)
        }
         */
    }
}
