use std::collections::HashSet;
use crate::interactions::syntax::interaction::{Interaction, LoopKind};

#[derive(Clone, PartialEq, Debug, Eq, Hash, PartialOrd, Ord)]
pub enum Operator {
    Alt,
    Par,
    Seq,
    Tensor,
    Loop(LoopKind),
    Cons,
    Coreg(Vec<usize>)
}
impl Operator{
    pub fn to_string(&self)->String{
        match self.clone(){
            Operator::Seq => "seq".to_string(),
            Operator::Par => "par".to_string(),
            Operator::Alt => "alt".to_string(),
            Operator::Loop(kind) => {
                match kind {
                    LoopKind::PInterleaving => "loopP".to_string(),
                    LoopKind::WWeakSeq => "loopW".to_string(),
                    LoopKind::HHeadFirstWS => "loopH".to_string(),
                    LoopKind::SStrictSeq => "loopS".to_string(),
                }
            },
            Operator::Tensor => "tensor".to_string(),
            Operator::Cons => "cons".to_string(),
            Operator::Coreg(vec) => {
                let mut ret = String::from("coreg");
                for v in vec {
                    ret.push_str(&format!("({})",v));
                }
                ret
            }
        }
    }
    pub fn string_operators(operators: &HashSet<Operator>)->String{
        let mut ret = String::from("");
        for op in operators {
            ret.push_str(&op.to_string());
            ret.push(' ');
        }

        ret
    }
}

impl Interaction{
    pub fn wrap_binary(int1: &Interaction, int2: &Interaction,op: &Operator) -> Interaction {
        match op {
            Operator::Seq =>{
                Interaction::Seq(Box::new(int1.clone()), Box::new(int2.clone()))
            },
            Operator::Par =>{
                Interaction::Par(Box::new(int1.clone()), Box::new(int2.clone()))
            },
            Operator::Alt =>{
                Interaction::Alt(Box::new(int1.clone()), Box::new(int2.clone()))
            },
            Operator::Tensor =>{
                Interaction::Tensor(Box::new(int1.clone()), Box::new(int2.clone()))
            },
            Operator::Coreg(vec) =>{
                Interaction::CoReg(vec.clone(),Box::new(int1.clone()),Box::new(int2.clone()))
            }
            _ => panic!("Unsupported operator for wrap_binary"),
        }
    }

    pub fn wrap_loop(kind: &LoopKind,int: &Interaction)-> Interaction {
        Interaction::Loop(kind.clone(),Box::new(int.clone()))
    }
}



