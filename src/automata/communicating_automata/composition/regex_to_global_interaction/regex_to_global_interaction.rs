use crate::automata::automaton::automaton::Automaton;
use crate::automata::communicating_automata::system_cautomata::cautomata::{CommunicatingAutomatonName, SystemCommunicatingAutomata};
use crate::automata::communicating_automata::system_cautomata::cautomata_actions::CommunicatingAutomataValuePassing;
use crate::interactions::syntax::general_context::GeneralContext;
use crate::interactions::syntax::interaction::{Interaction, LoopKind};
use crate::interactions::syntax::operators::Operator;
use crate::interactions::syntax::util::special_fold_int::{special_fold_recursive_alt_frags, special_fold_recursive_par_frags, special_fold_recursive_seq_frags};
use crate::regex_inf::regex::regex::Regex;

impl SystemCommunicatingAutomata{
    pub fn translate_regex_to_global_interaction_with_gates(gen_ctx:&GeneralContext,regex: &Regex<String>)->Result<Interaction,String>{
        match Self::regex_to_interaction_special2(&regex,
                                                 gen_ctx,
                                                  CommunicatingAutomataValuePassing::parse_value_passing){

            Ok(interaction)=> Ok(interaction),
            Err(e)=>{
                println!("Error: {:?}",e);
                Err(e)
            }
        }
    }

    pub fn regex_to_interaction_special2(regex: &Regex<String>, gen_ctx: &GeneralContext,action_parser: fn(general_context: &GeneralContext,action_str:&String)->Result<Interaction,String>) ->Result<Interaction,String>
    {
        match regex{
            Regex::Empty => Ok(Interaction::Empty),
            Regex::Epsilon=>Ok(Interaction::Empty),
            Regex::Literal(act_str)=>{
                if act_str.is_empty(){
                    Ok(Interaction::Empty)
                }
                else{
                    Ok(action_parser(gen_ctx, &act_str)?)

                }
            },
            Regex::Concat(factors)=>{

                let mut res_vec = vec![];

                for factor in factors.iter(){
                    res_vec.push(Self::regex_to_interaction_special2( &factor,gen_ctx,action_parser)?);
                }

                Ok(special_fold_recursive_seq_frags(&mut res_vec))
            },
            Regex::Or(factors)=>{
                let mut res_vec = vec![];

                for factor in factors.iter(){
                    res_vec.push(Self::regex_to_interaction_special2(&factor,gen_ctx,action_parser)?);
                }

                Ok(special_fold_recursive_alt_frags(&mut res_vec))
            },
            Regex::Int(factors)=>{
                let mut res_vec = vec![];

                for factor in factors.iter(){
                    res_vec.push(Self::regex_to_interaction_special2(&factor,gen_ctx,action_parser)?);
                }

                Ok(special_fold_recursive_par_frags(&mut res_vec))
            },
            Regex::Kleene(box1)=>{
                let reg = *box1.clone();

                Ok(Interaction::wrap_loop(&LoopKind::SStrictSeq,&Self::regex_to_interaction_special2(&reg,gen_ctx,action_parser)?))
            },
            Regex::Plus(box1)=>{
                /**/
                let reg = *box1.clone();
                let int = Self::regex_to_interaction_special2(&reg,gen_ctx,action_parser)?;
                let loop_int = Interaction::wrap_loop(&LoopKind::SStrictSeq,&int);

                Ok(Interaction::wrap_binary(&int,&loop_int,&Operator::Seq))


            },
            Regex::Qm(box1)=>{
                let reg = *box1.clone();
                Ok(Interaction::wrap_binary(&Self::regex_to_interaction_special2(&reg,gen_ctx,action_parser)?, &Interaction::Empty, &Operator::Alt))
            },
            _ =>{
                println!("{:?}",regex);
                panic!("Should not happen")
            }
        }
    }


    pub fn preprocess_automaton_for_petrify_vp(aut: &Automaton) ->Automaton{
        let mut new_automaton = aut.clone();
        for e in aut.graph.edge_indices(){
            //let (source,target) = aut.graph.edge_endpoints(e).unwrap();
            let weight = aut.graph.edge_weight(e).unwrap();
            let new_weight = Self::translate_label_into_petrify_vp(&weight).unwrap();
            //new_automaton.graph.update_edge(source,target,Self::translate_label_into_petrify(weight).unwrap());//Dangerous unwrap
            new_automaton.graph[e] = new_weight;
        }
        //We insert empty transitions to mark final states
        new_automaton.introduce_empty_transition()
    }

    pub fn translate_label_into_petrify_vp(input:&String) ->Option<String>{
        let (emitter, rest) = input
            .split_once("--")?;

        let (message, rest) = rest
            .split_once("->")?;

        let (receiver, gate) = if let Some((receiver, gate)) = rest.split_once('[') {
            let gate = gate
                .strip_suffix(']')?
                .parse::<usize>().ok()?;

            (receiver, Some(gate))
        } else {
            (rest, None)
        };
        if let Some(gt) = gate {
            Some(format!("{emitter}_{message}_{receiver}_{gt}"))
        }
        else{
             Some(format!("{emitter}_{message}_{receiver}"))
        }
    }

    pub fn translate_label_from_petrify_vp(input:&String) ->Option<String>{
        let parts: Vec<&str> = input.split('_').collect();
        if parts.len() == 4 {
            return Some(format!("{}--{}->{}[{}]", parts[0], parts[1], parts[2], parts[3]));
        }
        else if parts.len() == 3 {
            return Some(format!("{}--{}->{}", parts[0], parts[1], parts[2]));
        }
        None

    }

    pub fn translate_regex_because_of_petrify_vp(regex: &Regex<String>)->Regex<String>{
        match regex.clone() {
            Regex::Epsilon => Regex::Epsilon,
            Regex::Empty => Regex::Empty,
            Regex::Literal(s) => Regex::Literal(Self::translate_label_from_petrify_vp(&s).unwrap()),//Dangerous unwrap
            Regex::Concat(v)=>{
                let mut new_vec = Vec::new();
                for r in v{
                    new_vec.push(Self::translate_regex_because_of_petrify_vp(&r));
                }
                Regex::Concat(new_vec)
            },
            Regex::Or(v)=>{
                let mut new_vec = Vec::new();
                for r in v{
                    new_vec.push(Self::translate_regex_because_of_petrify_vp(&r));
                }
                Regex::Or(new_vec)
            },
            Regex::Int(v)=>{
                let mut new_vec = Vec::new();
                for r in v{
                    new_vec.push(Self::translate_regex_because_of_petrify_vp(&r));
                }
                Regex::Int(new_vec)
            },
            Regex::Kleene(b)=>{
                let r = *b.clone();
                let r_prime = Self::translate_regex_because_of_petrify_vp(&r);
                Regex::Kleene(Box::new(r_prime))
            },
            Regex::Plus(b)=>{
                let r = *b.clone();
                let r_prime = Self::translate_regex_because_of_petrify_vp(&r);
                Regex::Plus(Box::new(r_prime))
            },
            Regex::Qm(b)=>{
                let r = *b.clone();
                let r_prime = Self::translate_regex_because_of_petrify_vp(&r);
                Regex::Qm(Box::new(r_prime))
            },
            _ => panic!("Regex type not supported."),
        }
    }
}