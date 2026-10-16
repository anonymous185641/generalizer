use std::fmt;
use crate::terms::substitution::variable::Variable;
use crate::terms::term::Term;

#[derive(Clone, PartialEq, Debug, Eq,Hash,Ord,PartialOrd)]
pub struct AUT {
    pub x: Variable,
    pub t1: Term,
    pub t2: Term,
}


impl AUT {
    pub fn new(x: Variable, t1: Term, t2: Term) -> AUT {
        AUT { x, t1, t2 }
    }

    #[allow(dead_code)]
    pub fn from_pair(t1: &Term, t2: &Term) -> AUT {
        let x = Variable::fresh_variable();

        Self::new(x, t1.clone(), t2.clone())

    }



}


impl fmt::Display for AUT{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();

        result.push_str(&format!(" {}: {} =^= {}", self.x, self.t1, self.t2));
        write!(f, "{}", result.trim_end())
    }
}

