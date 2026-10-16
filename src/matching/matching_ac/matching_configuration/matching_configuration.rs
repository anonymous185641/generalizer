use crate::terms::function::{Function, FunctionSignature};
use crate::terms::substitution::substitution::Substitution;
use crate::terms::substitution::variable::Variable;
use crate::terms::term::Term;
#[derive(Clone, PartialEq, Debug, Eq, Hash,PartialOrd,Ord)]
pub struct MatchingConfiguration {
    pub unsolved_set: Vec<(Term,Term)>,
    pub partly_solved_set: Vec<(Variable, Variable, FunctionSignature, Term)>,
    pub solved_set: Vec<(Variable, Term)>,
}


impl MatchingConfiguration {
    pub fn new() -> MatchingConfiguration {
        MatchingConfiguration {
            unsolved_set: vec![],
            partly_solved_set: vec![],
            solved_set: vec![]
        }
    }
    
    pub fn init_conf(t1: &Term,t2: &Term) -> MatchingConfiguration {
        MatchingConfiguration {
            unsolved_set: vec![(t1.clone(),t2.clone())],
            partly_solved_set: vec![],
            solved_set: vec![]
        }
    }
    
    pub fn draw(&self,name : &str,temp_folder : &str) {
        todo!()
    }
    
    
    
    pub fn extract_substitution(&self)->Result<Substitution,String>{

        if self.unsolved_set.len() != 0{
            return Err("Unsolved set is not empty".to_string());
        }

        let mut sigma = Substitution::new();

        //Extraction from the solved set
        for (var, term) in &self.solved_set {
            sigma.insert(var, term);
        }

        //Extraction from the partly solved set
        for (x, y, func_sig, s) in &self.partly_solved_set {
            let temp_sigma = sigma.clone();
            //First binding
            let y_sigma = Term::Variable(y.clone()).apply_substitution(&temp_sigma);
            let f_y_sigma_s = Term::Function(Function::new(&func_sig,&vec![y_sigma.clone(),s.clone()]));
            sigma.insert(x, &f_y_sigma_s);

            //Remove y
            sigma.map.remove(&y);

        }

        Ok(sigma)
    }
}