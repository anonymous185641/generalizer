use pest::iterators::{Pair, Pairs};
use crate::global_counter::counter::fresh_number;
use crate::interactions::io::input::error::ParsingError;
use crate::interactions::io::input::hif::parser::Rule;
use crate::interactions::syntax::action::{Action, ActionType};
use crate::interactions::syntax::general_context::GeneralContext;
use crate::interactions::syntax::interaction::Interaction;




pub enum ParsedReference { //Please eliminate this crap
    LifelineRef(usize),
   // GateRef(usize)
}

pub fn parse_communication(gen_ctx : &GeneralContext, contents : &mut Pairs<Rule>) -> Result<Interaction,ParsingError> {
    let comm_act_content_pair : Pair<Rule>;
    let comm_act_target_pair : Pair<Rule>;
    let comm_act_gate_id_pair : Option<Pair<Rule>>;
    let mut origin_info : Option<ParsedReference> = None;
    // ***
    let first_pair = contents.next().unwrap();
    match first_pair.as_rule() {
        Rule::SD_COMMUNICATION_ORIGIN => {
            match parse_comm_act_origin(gen_ctx,first_pair) {
                Err(e) => {
                    return Err(e);
                },
                Ok( parsed_ref ) => {
                    origin_info = Some(parsed_ref);
                    comm_act_content_pair = contents.next().unwrap();
                    comm_act_target_pair = contents.next().unwrap();
                    comm_act_gate_id_pair = contents.next();
                }
            }
        },
        Rule::SD_COMMUNICATION_CONTENT => {
            comm_act_content_pair = first_pair;
            comm_act_target_pair = contents.next().unwrap();
            comm_act_gate_id_pair = contents.next();
        },
        _ => {
            panic!("what rule then ? : {:?}", first_pair.as_rule() );
        }
    }
    // ***
    match parse_comm_content(gen_ctx,comm_act_content_pair) {
        Err(e) => {
            return Err(e);
        },
        Ok( ms_id ) => {
            match origin_info {
                None => { // Here, we have a reception
                    match parse_comm_act_targets_as_lifelines(gen_ctx,comm_act_target_pair) {
                        Err(e) => {
                            return Err(e);
                        },
                        Ok( tar_lf_id) => {

                            if tar_lf_id == None{
                                return Err(ParsingError::MatchError(String::from("No Target")));
                            }
                            else {
                                let lf_id : usize = tar_lf_id.unwrap();

                                if comm_act_gate_id_pair != None {
                                    if let Ok(id) = parse_gate_id(comm_act_gate_id_pair.unwrap()){
                                        let action = Action::new_with_id(lf_id,ms_id,ActionType::Reception,id);

                                        return Ok( Interaction::Action(action) );
                                    }
                                }
                                let action = Action::new(lf_id,ms_id,ActionType::Reception);

                                return Ok( Interaction::Action(action) );

                            }

                        }
                    }
                },
                Some( parsed_ref ) => {
                    match parsed_ref {
                        ParsedReference::LifelineRef( lf_id ) => {
                            match parse_comm_act_targets_as_lifelines(gen_ctx,comm_act_target_pair) {
                                Err(e) => {
                                    return Err(e);
                                },
                                Ok( tar_ref) => {

                                    if tar_ref == None {//Emission

                                        if comm_act_gate_id_pair != None {
                                            if let Ok(id) = parse_gate_id(comm_act_gate_id_pair.unwrap()) {
                                                let action = Action::new_with_id(lf_id,ms_id,ActionType::Emission,id);
                                                return Ok( Interaction::Action(action) );
                                            }
                                        }
                                        let action = Action::new(lf_id,ms_id,ActionType::Emission);
                                        return Ok( Interaction::Action(action) );
                                    }
                                    else { //Value-passing
                                        let tar_id = tar_ref.unwrap();

                                        //This modification is for the sake of an experiment....
                                        let fresh_gate = fresh_number();

                                        let action1 = Action::new_with_id(lf_id,ms_id,ActionType::Emission,fresh_gate);
                                        let action2 = Action::new_with_id(tar_id,ms_id,ActionType::Reception,fresh_gate);
                                        return Ok( Interaction::Vp(action1,action2) );
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}




pub fn parse_comm_act_origin(gen_ctx : &GeneralContext, origin_pair : Pair<Rule>) -> Result<ParsedReference,ParsingError> {
    let origin_name_pair = origin_pair.into_inner().next().unwrap();
    let origin_name : String = origin_name_pair.as_str().chars().filter(|c| !c.is_whitespace()).collect();
    match gen_ctx.get_lf_id( &origin_name ) {
        None => {
            return Err( ParsingError::MissingLifelineOrGateDeclarationError(origin_name) );
        },
        Some( lf_id ) => {
            return Ok( ParsedReference::LifelineRef(lf_id) );
        }
    }
}



pub fn parse_comm_content(gen_ctx : &GeneralContext, comm_content_pair : Pair<Rule>) -> Result<usize,ParsingError> {
    let mut contents = comm_content_pair.into_inner();
    let first_pair = contents.next().unwrap();
    let got_ms_id =  match first_pair.as_rule() {
        Rule::STRING => {
            let ms_name : String = first_pair.as_str().chars().filter(|c| !c.is_whitespace()).collect();
            match gen_ctx.get_ms_id( &ms_name ) {
                None => {
                    return Err( ParsingError::MissingMessageDeclarationError( ms_name ) );
                },
                Some( ms_id ) => {
                     Some(ms_id)
                }
            }
        },
        _ => {
            panic!("what rule then ? : {:?}", first_pair.as_rule() );
        }
    };
    // ***
    match got_ms_id {
        None => {
            let second_pair = contents.next().unwrap();
            match second_pair.as_rule() {
                Rule::STRING => {
                    let ms_name : String = second_pair.as_str().chars().filter(|c| !c.is_whitespace()).collect();
                    match gen_ctx.get_ms_id( &ms_name ) {
                        None => {
                            return Err( ParsingError::MissingMessageDeclarationError( ms_name ) );
                        },
                        Some( ms_id ) => {
                            return Ok( ms_id );
                        }
                    }
                },
                _ => {
                    panic!("what rule then ? : {:?}", second_pair.as_rule() );
                }
            }
        },
        Some(ms_id) => {
            return Ok( ms_id );
        }
    }
}



pub fn parse_comm_act_targets_as_lifelines(gen_ctx : &GeneralContext, target_pair : Pair<Rule>) -> Result<Option<usize>,ParsingError> {
    let inner_pair = target_pair.into_inner().next().unwrap();
    match inner_pair.as_rule() {
        Rule::STRING => {
            let lf_name : String = inner_pair.as_str().chars().filter(|c| !c.is_whitespace()).collect();
            match gen_ctx.get_lf_id( &lf_name ) {
                None => {
                    return Err( ParsingError::MissingLifelineDeclarationError(lf_name) );
                },
                Some( lf_id ) => {
                    return Ok( Some(lf_id) );
                }
            }
        },
        Rule::ENVIRONMENT_TARGET => {
            return Ok( None );
        },
        _ => {
            panic!("what rule then ? : {:?}", inner_pair.as_rule() );
        }
    }
}

pub fn parse_gate_id(target_pair : Pair<Rule>) -> Result<usize,ParsingError> {
    //let inner_pair = target_pair.into_inner().next().unwrap();
    let inner_pair = target_pair.into_inner().next().unwrap();
    match inner_pair.as_rule() {
        Rule::GATE_ID =>{
            return Ok(inner_pair.as_str().parse::<usize>().unwrap());
        },
        _ => {
            panic!("what rule then ? : {:?}", inner_pair.as_rule() );
        }
    }
}
























