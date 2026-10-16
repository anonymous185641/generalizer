use crate::interactions::syntax::interaction::{Interaction, LoopKind};
use crate::interactions::syntax::operators::Operator;

pub fn transfo_loops_deduplicate(interaction : &Interaction) -> Vec<Interaction> {
        match *interaction {
            Interaction::Seq(ref i1,ref i2) =>{

                match (*i1.clone(), *i2.clone()) {
                    (Interaction::Loop(kind1,ref i11),Interaction::Loop(kind2,ref i21)) 
                    if *i11 == *i21 && kind1==kind2 && kind1==LoopKind::SStrictSeq => {
                        vec![*i1.clone()]
                    },
                    (Interaction::Loop(kind1,ref i11),Interaction::Seq(ref i21,ref i22)) => {

                        match *i21.clone() {
                            Interaction::Loop(kind2,ref i211) if *i11==*i211 && kind1==kind2 && kind1==LoopKind::SStrictSeq => {
                                vec![Interaction::wrap_binary(&*i1.clone(),&*i22.clone(),&Operator::Seq)]
                            },
                            _ =>{
                                return vec![];
                            }
                        }

                    }
                    _ =>{
                        return vec![];
                    }
                }
            },
            _ => {
                return vec![];
            }
        }
}