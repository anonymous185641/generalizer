use std::collections::HashSet;
use std::fmt;
use crate::terms::substitution::substitution::Substitution;
use crate::terms::substitution::variable::Variable;
use crate::terms::function::{Axioms, Function, FunctionSignature};

#[derive(Clone, PartialEq, Debug, Eq, Hash, Ord, PartialOrd)]
pub enum Term {
    Function(Function),
    Variable(Variable),
}



impl Term {

    pub fn is_special_constant(&self) -> bool {
        match self.clone(){
            Term::Function(f)=>{
                if f.args.len()!=0{
                    return false;
                }

                if f.signature.arity==0 && f.signature.axioms.contains(&Axioms::SpecialConst){
                    return true;
                }

                false
            },
            Term::Variable(_)=>false
        }
    }

    pub fn is_variable(&self) -> bool {
        match self.clone(){
            Term::Variable(_)=>{ true },
            _=>false
        }
    }

    pub fn is_constant_symbol(&self) -> bool {
        match self.clone(){
            Term::Variable(_)=>{ false },
            Term::Function(f)=>{ 
                f.signature.arity==0
            }
        }   
    }
    pub fn is_head_function_free(&self)->bool{
        match self.clone(){
            Term::Function(f)=>{
                if f.signature.axioms.is_empty()
                {
                    return true;
                }

                false
            },
            Term::Variable(_)=>false
        }
    }
    pub fn is_head_function_has_unit(&self)->bool{
        match self.clone(){
            Term::Function(f)=>{
                if f.signature.axioms.contains(&Axioms::U)
                {
                    return true;
                }

                false
            },
            Term::Variable(_)=>false
        }
    }
    pub fn is_head_function_commutative(&self) -> bool {
        match self.clone(){
            Term::Function(f)=>{
                if f.signature.axioms.contains(&Axioms::C)
                && !f.signature.axioms.contains(&Axioms::A){
                    return true;
                }

                false
            },
            Term::Variable(_)=>false
        }
    }

    pub fn is_head_function_associative(&self) -> bool {
        match self.clone(){
            Term::Function(f)=>{
                if f.signature.axioms.contains(&Axioms::A)
                    && !f.signature.axioms.contains(&Axioms::C){
                    return true;
                }

                false
            },
            Term::Variable(_)=>false
        }
    }

    pub fn is_head_function_associative_commutative(&self) -> bool {
        match self.clone(){
            Term::Function(f)=>{
                if f.signature.axioms.contains(&Axioms::C)
                    && f.signature.axioms.contains(&Axioms::A){
                    return true;
                }

                false
            },
            Term::Variable(_)=>false
        }
    }
    
    
    pub fn get_special_constants(&self) -> HashSet<String> {
        match self.clone() {
            Term::Variable(_)=>{
                HashSet::new()
            },
            Term::Function(f)=>{
                if self.is_special_constant(){
                    return HashSet::from([self.head_symbol_signature().name.clone()]);
                }

                let mut ret = HashSet::new();

                for arg in f.args{
                    ret.extend(arg.get_special_constants());
                }

                ret
            }
        }
    }
    
    /*
    pub fn get_special_constants(&self) -> HashSet<Term> {
        match self.clone() {
            Term::Variable(_)=>{
                HashSet::new()
            },
            Term::Function(f)=>{
                if self.is_special_constant(){
                    return HashSet::from([self.clone()]);
                }

                let mut ret = HashSet::new();

                for arg in f.args{
                    ret.extend(arg.get_special_constants());
                }

                ret
            }
        }
    }
     */

    pub fn apply_substitution(&self, sub: &Substitution)-> Term {
        match self.clone() {
            Term::Variable(var) => {
                if let Some(t) = sub.map.get(&var){
                    t.clone()
                }else{
                    self.clone()
                }
            },
            Term::Function(f) => {
                let mut args = Vec::new();

                for arg in &f.args {
                    args.push(arg.apply_substitution(sub));
                }

                Term::Function(Function::new(&f.signature,&args))
            }
        }
    }

    pub fn head(&self) -> String {
        match self.clone() {
            Term::Variable(v)=> v.name(),
            Term::Function(f)=> f.signature.name
        }
    }

    pub fn head_symbol_signature(&self) -> FunctionSignature {
        match self.clone() {
            Term::Variable(_)=> panic!("Applied head_ground to a non-gound term"),
            Term::Function(f)=> f.signature
        }
    }


    pub fn lifelines_ids(&self) -> Vec<usize>{
        match self.clone() {
            Term::Variable(_)=> vec![],
            Term::Function(f)=> {

                if f.signature.arity == 0{
                    return f.signature.lifeline_ids.unwrap_or(vec![]).clone();
                }
                else{
                    let mut ret = Vec::new();
                    for arg in &f.args{
                        ret.extend(arg.lifelines_ids());
                    }
                    ret
                }

            }
        }
    }

    pub fn assoc_flatten(&self)->Term{

        match self.clone(){
            Term::Variable(_x) => self.clone(),
            Term::Function(f)=>{
                let mut new_args = Vec::new();
                for arg in f.args{
                    match arg.clone(){
                        Term::Variable(_x) => {new_args.push(arg);},
                        Term::Function(g)=>{
                            if g.signature== f.signature{
                                //First we flatten it
                                let new_arg = arg.assoc_flatten();

                                //Then we take its arguments
                                if let Term::Function(new_g) = new_arg{
                                    new_args.extend(new_g.args.clone());
                                }
                            }
                            else{
                                new_args.push(arg);
                            }
                        }
                    }

                }
                Term::Function(Function::new(&f.signature, &new_args))
            }


        }


    }

    pub fn assoc_wrap(sig: &FunctionSignature,args: &Vec<Term>)->Term{
        if sig.axioms.contains(&Axioms::A) && args.len() == 1{
            return args[0].clone();
        }

        Term::Function(Function::new(sig,args))
    }
}


impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        if self.is_special_constant(){
           // write!(f, "{}", self.head().red())
            write!(f, "{}", self.head())
        }
        else{
            match self {
                Term::Function(func) => write!(f, "{}", func),
                Term::Variable(v) => write!(f, "{}", v)
            }
        }

    }
}