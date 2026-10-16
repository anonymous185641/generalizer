use crate::terms::function::{Axioms, FunctionSignature, Signature};
use crate::terms::term::Term;

impl Term{
    pub fn get_signature(&self) -> Signature{
         match self{
             Term::Variable(_)=> vec![],
             Term::Function(f)=>{
                 let mut sig = Vec::new();

                 sig.push(f.signature.clone());

                 for arg in f.args.iter(){
                     sig.append(&mut arg.get_signature());
                 }
                 sig.sort();
                 sig.dedup();
                 sig
             }
         }
    }

    pub fn get_interaction_signature() -> Signature{
        let seq_sig = FunctionSignature::new("seq".to_string(), 2, vec![Axioms::A, Axioms::U]);
        let par_sig = FunctionSignature::new("par".to_string(), 2, vec![Axioms::A, Axioms::C, Axioms::U]);
        let alt_sig = FunctionSignature::new("alt".to_string(), 2, vec![Axioms::A, Axioms::C]);
        let loop_sig = FunctionSignature::new("loopS".to_string(), 1, vec![]);
        let empty = FunctionSignature::new("Empty".to_string(), 0, vec![]);

        vec![seq_sig,par_sig,alt_sig,loop_sig,empty]
    }

    pub fn special_get_interaction_signature(terms: &Vec<Term>) -> Signature{
        let mut sig = Vec::new();

        for term in terms.iter(){
            sig.extend(term.get_signature());
        }
        sig.extend(Self::get_interaction_signature());

        sig.sort();
        sig.dedup();
        sig
    }

}