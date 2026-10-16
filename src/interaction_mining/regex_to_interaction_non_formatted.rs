


use itertools::Itertools;
use crate::interactions::syntax::action::{Action, ActionType};
use crate::interactions::syntax::general_context::GeneralContext;
use crate::interactions::syntax::interaction::{Interaction, LoopKind};
use crate::interactions::syntax::operators::Operator;
use crate::interactions::syntax::util::special_fold_int::{special_fold_recursive_alt_frags, special_fold_recursive_par_frags, special_fold_recursive_seq_frags};
use crate::regex_inf::regex::regex::Regex;

impl Interaction {

    pub fn regex_to_interaction_with_name(gen_ctx: &GeneralContext, regex: &Regex<String>) ->Result<Interaction,String>{
        //match regex.simplify(){
        match regex{
            Regex::Empty => Ok(Interaction::Empty),
            Regex::Epsilon=>Ok(Interaction::Empty),
            Regex::Literal(act_str)=>{
                if act_str.is_empty(){
                    Ok(Interaction::Empty)
                }
                else{
                    Interaction::parse_action_with_names(gen_ctx, &act_str)
                    
                }
            },
            Regex::Concat(factors)=>{

                let mut res_vec = vec![];

                for factor in factors.iter(){
                    res_vec.push(Self::regex_to_interaction_with_name(gen_ctx, &factor)?)
                }

                Ok(special_fold_recursive_seq_frags(&mut res_vec))
            },
            Regex::Or(factors)=>{
                let mut res_vec = vec![];

                for factor in factors.iter(){
                    res_vec.push(Self::regex_to_interaction_with_name(gen_ctx, &factor)?)
                }

                Ok(special_fold_recursive_alt_frags(&mut res_vec))
            },
            Regex::Int(factors)=>{
                let mut res_vec = vec![];

                for factor in factors.iter(){
                    res_vec.push(Self::regex_to_interaction_with_name(gen_ctx, &factor)?)
                }

                Ok(special_fold_recursive_par_frags(&mut res_vec))
            },
            Regex::Kleene(box1)=>{
                let reg = *box1.clone();

                Ok(Interaction::wrap_loop(&LoopKind::SStrictSeq,&Self::regex_to_interaction_with_name(gen_ctx, &reg)?))
            },
            Regex::Plus(box1)=>{
                let reg = *box1.clone();
                let int = Self::regex_to_interaction_with_name(gen_ctx, &reg)?;
                let loop_int = Interaction::wrap_loop(&LoopKind::SStrictSeq,&int);

                Ok(Interaction::wrap_binary(&int,&loop_int,&Operator::Seq))
            },
            Regex::Qm(box1)=>{
                let reg = *box1.clone();
                Ok(Interaction::wrap_binary(&Self::regex_to_interaction_with_name(gen_ctx, &reg)?, &Interaction::Empty, &Operator::Alt))
            },
            _ =>{
                println!("{:?}",regex);
                panic!("Should not happen")
            }
        }
    }


    fn parse_action_with_names(gen_ctx: &GeneralContext, action_str: &String) ->Result<Interaction,String>{
        // Expected formats:
        //   l_name!m_name  → emission
        //   l_name?m_name  → reception
        //

        match Self::simple_parse(action_str){
            Some((left,'!',right))=>{
                let lf_id = gen_ctx.get_lf_id(left).ok_or("Invalid left lf_id")?;
                let ms_id = gen_ctx.get_ms_id(right).ok_or("Invalid right ms_id")?;
                Ok(Interaction::Action( Action {
                    lf_id,
                    ms_id,
                    action_type: ActionType::Emission,
                    gate_id: None,
                }))
            },
            Some((left,'?',right))=>{
                let lf_id = gen_ctx.get_lf_id(left).ok_or("Invalid left lf_id")?;
                let ms_id = gen_ctx.get_ms_id(right).ok_or("Invalid right ms_id")?;
                Ok(Interaction::Action( Action {
                    lf_id,
                    ms_id,
                    action_type: ActionType::Reception,
                    gate_id: None,
                }))
            },
            _ => {
                return Err("Invalid action format".into());
            }
        }

    }

    fn simple_parse(input:&str)->Option<(&str,char,&str)>{
        if let Some((left,right)) = input.split_once('!') {
            return Some((left, '!', right));
        }
        if let Some((left,right)) = input.split_once('?') {
            return Some((left, '?', right));
        }
        None
    }


}