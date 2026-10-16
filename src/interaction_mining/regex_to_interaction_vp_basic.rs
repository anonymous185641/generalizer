use itertools::Itertools;
use crate::interactions::syntax::action::{Action, ActionType};
use crate::interactions::syntax::general_context::GeneralContext;
use crate::interactions::syntax::interaction::{Interaction, LoopKind};
use crate::interactions::syntax::operators::Operator;
use crate::interactions::syntax::traces::sync_trace::ValuePassing;
use crate::interactions::syntax::util::special_fold_int::{special_fold_recursive_alt_frags, special_fold_recursive_par_frags, special_fold_recursive_seq_frags};
use crate::regex_inf::regex::regex::Regex;

impl Interaction {

    pub fn regex_to_interaction_vp(regex: &Regex<String>) ->Result<Interaction,String>{
        //match regex.simplify(){
        match regex{
            Regex::Empty => Ok(Interaction::Empty),
            Regex::Epsilon=>Ok(Interaction::Empty),
            Regex::Literal(act_str)=>{
                if act_str.is_empty(){
                    Ok(Interaction::Empty)
                }
                else{
                    Ok(ValuePassing::parse_vp(&act_str)?.to_interaction())
                }
            },
            Regex::Concat(factors)=>{

                let mut res_vec = vec![];

                for factor in factors.iter(){
                    res_vec.push(Self::regex_to_interaction_vp(&factor)?)
                }

                Ok(special_fold_recursive_seq_frags(&mut res_vec))
            },
            Regex::Or(factors)=>{
                let mut res_vec = vec![];

                for factor in factors.iter(){
                    res_vec.push(Self::regex_to_interaction_vp(&factor)?)
                }

                Ok(special_fold_recursive_alt_frags(&mut res_vec))
            },
            Regex::Int(factors)=>{
                let mut res_vec = vec![];

                for factor in factors.iter(){
                    res_vec.push(Self::regex_to_interaction_vp(&factor)?)
                }

                Ok(special_fold_recursive_par_frags(&mut res_vec))
            },
            Regex::Kleene(box1)=>{
                let reg = *box1.clone();

                Ok(Interaction::wrap_loop(&LoopKind::SStrictSeq,&Self::regex_to_interaction_vp(&reg)?))
            },
            Regex::Plus(box1)=>{
                /**/
                let reg = *box1.clone();
                let int = Self::regex_to_interaction_vp(&reg)?;
                let loop_int = Interaction::wrap_loop(&LoopKind::SStrictSeq,&int);

                Ok(Interaction::wrap_binary(&int,&loop_int,&Operator::Seq))

                /*
                let reg = *box1.clone();

                Ok(Interaction::wrap_loop(&Self::regex_to_interaction_vp(&reg)?))

                 */
            },
            Regex::Qm(box1)=>{
                let reg = *box1.clone();
                Ok(Interaction::wrap_binary(&Self::regex_to_interaction_vp(&reg)?, &Interaction::Empty, &Operator::Alt))
            },
            _ =>{
                println!("{:?}",regex);
                panic!("Should not happen")
            }
        }
    }


    

}