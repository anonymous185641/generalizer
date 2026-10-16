use std::fmt;
use crate::terms::term::Term;

#[derive(Clone, PartialEq, Eq,Hash, Debug, Ord, PartialOrd)]
pub enum Axioms {
    SpecialConst,
    A,
    C,
    U
}

impl Axioms {
    pub fn acu()->Vec<Axioms>{
        vec![Axioms::A,Axioms::C,Axioms::U]
    }
}

#[derive(Clone, PartialEq,Eq, Debug, Hash, Ord, PartialOrd)]
pub struct FunctionSignature {
    pub name: String,
    pub arity: usize,
    pub axioms: Vec<Axioms>,
    pub lifeline_ids: Option<Vec<usize>>
}
#[derive(Clone, PartialEq, Debug, Eq, Hash, Ord, PartialOrd)]
pub struct Function {
    pub signature: FunctionSignature,
    pub args: Vec<Term>,
}

pub type Signature = Vec<FunctionSignature>;


impl FunctionSignature {
    pub fn new(name: String, arity: usize,axioms: Vec<Axioms>) -> FunctionSignature {
        FunctionSignature{name, arity,axioms,lifeline_ids:None}
    }

    pub fn new_with_lfs(name: String,arity: usize,axioms: Vec<Axioms>, lf_ids:Vec<usize>) -> FunctionSignature {
        FunctionSignature{name, arity:0,axioms,lifeline_ids:Some(lf_ids)}
    }
    pub fn get_unit(&self)->Term{

        if self.name.as_str()=="seq"||self.name.as_str()=="par"||self.name.as_str()=="tensor"||self.name.contains("coreg") {
            let sig = FunctionSignature::new("Empty".to_string(),0,vec![]);
            let f = Function::new(&sig,&vec![]);
            Term::Function(f)
        }
        else{
            let name = format!("U_{}", self.name);
            let sig = FunctionSignature::new(name,0,vec![]);
            Term::Function(Function::new(&sig,&vec![]))
        }
        /*
        match self.name.as_str(){
            "seq"|"par"|"tensor"=>{
                let sig = FunctionSignature::new("Empty".to_string(),0,vec![]);
                let f = Function::new(&sig,&vec![]);
                Term::Function(f)
            },
            _ =>{
                let name = format!("U_{}", self.name);
                let sig = FunctionSignature::new(name,0,vec![]);
                Term::Function(Function::new(&sig,&vec![]))
            }
        }
        
         */

    }

}

impl Function {
    pub fn new(signature: &FunctionSignature, args: &Vec<Term>) -> Function {
        Function{signature: signature.clone(), args: args.clone()}
    }

    pub fn get_unit(&self)->Term{
        self.signature.get_unit()
    }
}





impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.signature.arity == 0 {
            write!(f, "{}", self.signature.name)
        }
        else {
            let args_str = self.args.iter().map(|arg| arg.to_string()).collect::<Vec<String>>().join(", ");

            write!(f, "{}({})", self.signature.name, args_str)
        }
    }
}



impl fmt::Display for Axioms {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Axioms::SpecialConst=>{
                write!(f, "{}", "SpecialConst")
            },
            Axioms::A=>{
                write!(f, "{}", "A")
            },
            Axioms::C=>{
                write!(f, "{}", "C")
            },
            Axioms::U=>{
                write!(f, "{}", "U")
            }
        }
    }
}