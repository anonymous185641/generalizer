use std::collections::BTreeMap;
use itertools::Itertools;
use crate::automata::communicating_automata::composition::caut_composition_error::CommunicatingAutomataCompositionError;
use crate::automata::communicating_automata::system_cautomata::cautomata::{CommunicatingAutomatonName, SystemCommunicatingAutomata};
use crate::automata::communicating_automata::system_cautomata::cautomata_actions::CommunicatingAutomataAction;
use crate::global_counter::counter::fresh_number;
use crate::interactions::syntax::action::{Action, ActionType};
use crate::interactions::syntax::general_context::GeneralContext;
use crate::interactions::syntax::interaction::{Interaction, LoopKind};
use crate::interactions::syntax::operators::Operator;
use crate::interactions::syntax::util::special_fold_int::{special_fold_recursive_alt_frags, special_fold_recursive_par_frags, special_fold_recursive_seq_frags};
use crate::regex_inf::regex::regex::Regex;

impl SystemCommunicatingAutomata {

    pub fn translate_regex_to_interaction_with_gates(gen_ctx: &GeneralContext,regex_family: &Vec<(CommunicatingAutomatonName,Regex<String>)>)->Result<Vec<Interaction>, CommunicatingAutomataCompositionError>{

        let mut interactions: Vec<Interaction> = Vec::new();

        for (aut_name,regex) in regex_family.iter(){
            match Self::regex_to_interaction_special(&regex,
                                                     aut_name,
                                                     gen_ctx,
                                                     Self::parse_actions_caut){

                Ok(interaction)=> interactions.push(interaction),
                Err(e)=>{
                    println!("Error: {:?}",e);
                    panic!("Should not happen")
                }

            }
        }

        Ok(interactions)
    }

    pub fn parse_actions_caut(name_aut: &CommunicatingAutomatonName,gen_ctx:&GeneralContext,label:&String) -> Result<Interaction,String>{
        let caut_action = CommunicatingAutomataAction::parse_label(label)?;


        match caut_action{
            CommunicatingAutomataAction::Emission(e)=>{
                let msg = e.message.clone();

                let gate = if let Some(gate) = e.gate{
                    Some(gate.0)
                }else{
                    None
                };
                let act = Action::new_with_id_raw(gen_ctx.get_lf_id(&name_aut.0).unwrap(),gen_ctx.get_ms_id(&msg).unwrap(),ActionType::Emission,gate);
                Ok(Interaction::Action(act))
            },
            CommunicatingAutomataAction::Reception(r)=>{
                let msg = r.message.clone();

                let gate = if let Some(gate) = r.gate{
                    Some(gate.0)
                }else{
                    None
                };
                let act = Action::new_with_id_raw(gen_ctx.get_lf_id(&name_aut.0).unwrap(),gen_ctx.get_ms_id(&msg).unwrap(),ActionType::Reception,gate);
                Ok(Interaction::Action(act))
            }
        }
    }

    pub fn regex_to_interaction_special(regex: &Regex<String>,aut_name: &CommunicatingAutomatonName, gen_ctx: &GeneralContext,action_parser: fn(aut_name:&CommunicatingAutomatonName,general_context: &GeneralContext,action_str:&String)->Result<Interaction,String>) ->Result<Interaction,String>
    {
        match regex{
            Regex::Empty => Ok(Interaction::Empty),
            Regex::Epsilon=>Ok(Interaction::Empty),
            Regex::Literal(act_str)=>{
                if act_str.is_empty(){
                    Ok(Interaction::Empty)
                }
                else{
                    Ok(action_parser(aut_name,gen_ctx, &act_str)?)

                }
            },
            Regex::Concat(factors)=>{

                let mut res_vec = vec![];

                for factor in factors.iter(){
                    res_vec.push(Self::regex_to_interaction_special( &factor,aut_name,gen_ctx,action_parser)?);
                }

                Ok(special_fold_recursive_seq_frags(&mut res_vec))
            },
            Regex::Or(factors)=>{
                let mut res_vec = vec![];

                for factor in factors.iter(){
                    res_vec.push(Self::regex_to_interaction_special(&factor,aut_name,gen_ctx,action_parser)?);
                }

                Ok(special_fold_recursive_alt_frags(&mut res_vec))
            },
            Regex::Int(factors)=>{
                let mut res_vec = vec![];

                for factor in factors.iter(){
                    res_vec.push(Self::regex_to_interaction_special(&factor,aut_name,gen_ctx,action_parser)?);
                }

                Ok(special_fold_recursive_par_frags(&mut res_vec))
            },
            Regex::Kleene(box1)=>{
                let reg = *box1.clone();

                Ok(Interaction::wrap_loop(&LoopKind::SStrictSeq,&Self::regex_to_interaction_special(&reg,aut_name,gen_ctx,action_parser)?))
            },
            Regex::Plus(box1)=>{
                /**/
                let reg = *box1.clone();
                let int = Self::regex_to_interaction_special(&reg,aut_name,gen_ctx,action_parser)?;
                let loop_int = Interaction::wrap_loop(&LoopKind::SStrictSeq,&int);

                Ok(Interaction::wrap_binary(&int,&loop_int,&Operator::Seq))


            },
            Regex::Qm(box1)=>{
                let reg = *box1.clone();
                Ok(Interaction::wrap_binary(&Self::regex_to_interaction_special(&reg,aut_name,gen_ctx,action_parser)?, &Interaction::Empty, &Operator::Alt))
            },
            _ =>{
                println!("{:?}",regex);
                panic!("Should not happen")
            }
        }
    }

}










