use itertools::Itertools;
use crate::interactions::syntax::action::{Action, ActionType};
use crate::interactions::syntax::general_context::GeneralContext;
use crate::interactions::syntax::interaction::{Interaction, LoopKind};
use crate::interactions::syntax::operators::Operator;
use crate::interactions::syntax::util::special_fold_int::{special_fold_recursive_alt_frags, special_fold_recursive_par_frags, special_fold_recursive_seq_frags};
use crate::regex_inf::regex::regex::Regex;

impl Interaction {
    
    pub fn regex_to_interaction_generic(regex: &Regex<String>, gen_ctx: &GeneralContext, action_parser: fn(general_context: &GeneralContext,action_str:&String)->Result<Action,String>) ->Result<Interaction,String>
   {
        match regex{
            Regex::Empty => Ok(Interaction::Empty),
            Regex::Epsilon=>Ok(Interaction::Empty),
            Regex::Literal(act_str)=>{
                if act_str.is_empty(){
                    Ok(Interaction::Empty)
                }
                else{
                    Ok(Interaction::Action(action_parser(gen_ctx, &act_str)?))

                }
            },
            Regex::Concat(factors)=>{

                let mut res_vec = vec![];

                for factor in factors.iter(){
                    res_vec.push(Self::regex_to_interaction_generic( &factor,gen_ctx,action_parser)?);
                }

                Ok(special_fold_recursive_seq_frags(&mut res_vec))
            },
            Regex::Or(factors)=>{
                let mut res_vec = vec![];

                for factor in factors.iter(){
                    res_vec.push(Self::regex_to_interaction_generic(&factor,gen_ctx,action_parser)?);
                }

                Ok(special_fold_recursive_alt_frags(&mut res_vec))
            },
            Regex::Int(factors)=>{
                let mut res_vec = vec![];

                for factor in factors.iter(){
                    res_vec.push(Self::regex_to_interaction_generic(&factor,gen_ctx,action_parser)?);
                }

                Ok(special_fold_recursive_par_frags(&mut res_vec))
            },
            Regex::Kleene(box1)=>{
                let reg = *box1.clone();

                Ok(Interaction::wrap_loop(&LoopKind::SStrictSeq,&Self::regex_to_interaction_generic(&reg,gen_ctx,action_parser)?))
            },
            Regex::Plus(box1)=>{
                /**/
                let reg = *box1.clone();
                let int = Self::regex_to_interaction_generic(&reg,gen_ctx,action_parser)?;
                let loop_int = Interaction::wrap_loop(&LoopKind::SStrictSeq,&int);

                Ok(Interaction::wrap_binary(&int,&loop_int,&Operator::Seq))


                /*
                //TEMPORARY, I AM GONNA TREAT IT AS A KLEENE FOR A BIT
                let reg = *box1.clone();
                Ok(Interaction::wrap_loop(&Self::regex_to_interaction_generic(&reg,gen_ctx,action_parser)?))

                 */


            },
            Regex::Qm(box1)=>{
                let reg = *box1.clone();
                Ok(Interaction::wrap_binary(&Self::regex_to_interaction_generic(&reg,gen_ctx,action_parser)?, &Interaction::Empty, &Operator::Alt))
            },
            _ =>{
                println!("{:?}",regex);
                panic!("Should not happen")
            }
        }
    }

    pub fn regex_to_interaction_formatted(regex: &Regex<String>) ->Result<Interaction,String>{
        //match regex.simplify(){
        match regex{
            Regex::Empty => Ok(Interaction::Empty),
            Regex::Epsilon=>Ok(Interaction::Empty),
            Regex::Literal(act_str)=>{
                if act_str.is_empty(){
                   Ok(Interaction::Empty)
                }
                else{
                    Interaction::parse_action_formatted(&act_str)
                }
            },
            Regex::Concat(factors)=>{

                let mut res_vec = vec![];

                for factor in factors.iter(){
                    res_vec.push(Self::regex_to_interaction_formatted(&factor)?)
                }

                Ok(special_fold_recursive_seq_frags(&mut res_vec))
            },
            Regex::Or(factors)=>{
                let mut res_vec = vec![];

                for factor in factors.iter(){
                    res_vec.push(Self::regex_to_interaction_formatted(&factor)?)
                }

                Ok(special_fold_recursive_alt_frags(&mut res_vec))
            },
            Regex::Int(factors)=>{
                let mut res_vec = vec![];

                for factor in factors.iter(){
                    res_vec.push(Self::regex_to_interaction_formatted(&factor)?)
                }

                Ok(special_fold_recursive_par_frags(&mut res_vec))
            },
            Regex::Kleene(box1)=>{
                let reg = *box1.clone();

                Ok(Interaction::wrap_loop(&LoopKind::SStrictSeq,&Self::regex_to_interaction_formatted(&reg)?))
            },
            Regex::Plus(box1)=>{
                /**/
                let reg = *box1.clone();
                let int = Self::regex_to_interaction_formatted(&reg)?;
                let loop_int = Interaction::wrap_loop(&LoopKind::SStrictSeq,&int);

                Ok(Interaction::wrap_binary(&int,&loop_int,&Operator::Seq))

                /*
                let reg = *box1.clone();

                Ok(Interaction::wrap_loop(&Self::regex_to_interaction_formatted(&reg)?))

                 */
            },
            Regex::Qm(box1)=>{
                let reg = *box1.clone();
                Ok(Interaction::wrap_binary(&Self::regex_to_interaction_formatted(&reg)?, &Interaction::Empty, &Operator::Alt))
            },
            _ =>{
                println!("{:?}",regex);
                panic!("Should not happen")
            }
        }
    }


    fn parse_action_formatted(action_str: &String) ->Result<Interaction,String>{
        // Expected formats:
        //   l<lf>!m<ms>  → emission
        //   l<lf>?m<ms>  → reception
        //
        // But <lf> and <ms> in the string are 1-based, so we convert to 0-based.

        if !action_str.starts_with('l') {
            return Err("Action must start with 'l'".into());
        }

        // Strip initial 'l'
        let rest = &action_str[1..];

        // Find the separator (! or ?)
        let (lf_part, remainder) = match rest.split_once(['!', '?']) {
            Some(parts) => parts,
            None => return Err("Missing '!' or '?'".into()),
        };

        // Parse lf_id (1-based → 0-based)
        let lf_id: usize = lf_part
            .parse()
            .map_err(|_| "Invalid lf_id number".to_string())?;
        if lf_id == 0 {
            return Err("lf_id must be >= 1".into());
        }
        let lf_id = lf_id - 1;

        // Determine action type based on the separator
        let action_type = if rest.contains('!') {
            ActionType::Emission
        } else {
            ActionType::Reception
        };

        // Remainder must start with "m"
        let remainder = remainder.strip_prefix('m')
            .ok_or("Expected 'm' before ms_id")?;

        // Parse ms_id (1-based → 0-based)
        let ms_id: usize = remainder
            .parse()
            .map_err(|_| "Invalid ms_id number".to_string())?;
        if ms_id == 0 {
            return Err("ms_id must be >= 1".into());
        }
        let ms_id = ms_id - 1;

        Ok(Interaction::Action( Action {
            lf_id,
            ms_id,
            action_type,
            gate_id: None,
        }))
    }



}