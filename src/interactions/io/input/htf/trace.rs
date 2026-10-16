/*
Copyright 2020 Erwan Mahe (github.com/erwanM974)

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

use std::collections::{BTreeSet, HashSet};

use pest::iterators::Pair;




#[allow(unused_imports)]
use pest::Parser;
use crate::interactions::io::input::error::ParsingError;
use crate::interactions::io::input::htf::parser::Rule;
use crate::interactions::syntax::action::{Action, ActionType};
use crate::interactions::syntax::general_context::GeneralContext;
use crate::interactions::syntax::traces::trace::Trace;



pub fn trace_sequence_from_pair(gen_ctx : &GeneralContext,
                                trace_sequence_pair : Pair<Rule>,
                                unavailable_lifelines : &BTreeSet<usize>,
                                lifelines : &mut BTreeSet<usize>,
                                add_lfs : bool) -> Result<Trace,ParsingError> {
    let mut trace : Trace = vec![];
    for trace_elt_pair in trace_sequence_pair.into_inner() {
        match trace_element_from_pair(gen_ctx,trace_elt_pair,unavailable_lifelines,lifelines,add_lfs) {
            Err(e) => {
                return Err(e);
            }
            Ok(trace_elt) => {
                trace.push(trace_elt);
            }
        }
    }
    return Ok( trace );
}


pub fn trace_element_from_pair(gen_ctx : &GeneralContext,
                               trace_elt_pair : Pair<Rule>,
                               unavailable_lifelines : &BTreeSet<usize>,
                               lifelines : &mut BTreeSet<usize>,
                               add_lfs : bool) -> Result<Action,ParsingError> {
    match trace_elt_pair.as_rule() {
        Rule::TRACE_ACTION => {
            match get_trace_action(gen_ctx,trace_elt_pair,unavailable_lifelines,lifelines,add_lfs) {
                Err(e) => {return Err(e);},
                Ok( action ) => {
                    return Ok(action);
                }
            }
        },
        _ => {
            panic!("what rule then ? : {:?}", trace_elt_pair.as_rule() );
        }
    }
}

fn get_trace_action(gen_ctx : &GeneralContext,
                    action_pair : Pair<Rule>,
                    unavailable_lifelines : &BTreeSet<usize>,
                    lifelines : &mut BTreeSet<usize>,
                    add_lfs : bool)-> Result<Action,ParsingError>  {
    match trace_action_from_text(gen_ctx,action_pair) {
        Err(e) => {
            return Err(e);
        },
        Ok( action ) => {
            if unavailable_lifelines.contains(&action.lf_id) {
                return Err( ParsingError::NonDisjointTraceComponents );
            } else {
                if add_lfs {
                    lifelines.insert( action.lf_id);
                } else {
                    if !lifelines.contains( &action.lf_id ) {
                        return Err( ParsingError::IllDefinedTraceComponents(format!("lifeline of action {:?} not in predefined co-localisation {:?}",
                                                                                         action,
                                                                                         lifelines)) );
                    }
                }
            }
            return Ok( action );
        }
    }
}


fn trace_action_from_text(gen_ctx : &GeneralContext,
                          action_pair : Pair<Rule>) -> Result<Action,ParsingError> {
    let mut contents = action_pair.into_inner();
    // ***
    let lf_pair : Pair<Rule> = contents.next().unwrap();
    let lf_name : String  = lf_pair.as_str().chars().filter(|c| !c.is_whitespace()).collect();
    // ***
    let got_lf_id : usize;
    match gen_ctx.get_lf_id(&lf_name) {
        None => {
            return Err( ParsingError::MissingLifelineDeclarationError(lf_name) );
        },
        Some( lf_id ) => {
            got_lf_id = lf_id;
        }
    }
    // ***
    let act_kind_pair : Pair<Rule> = contents.next().unwrap();
    let act_kind : ActionType;
    match act_kind_pair.as_rule() {
        Rule::TRACE_EMISSION_SYMBOL => {
            act_kind = ActionType::Emission;
        },
        Rule::TRACE_RECEPTION_SYMBOL => {
            act_kind = ActionType::Reception;
        },
        _ => {
            panic!("what rule then ? : {:?}", act_kind_pair.as_rule() );
        }
    }
    // ***
    let ms_pair : Pair<Rule> = contents.next().unwrap();
    let ms_name : String  = ms_pair.as_str().chars().filter(|c| !c.is_whitespace()).collect();
    // ***
    let got_ms_id : usize;
    match gen_ctx.get_ms_id(&ms_name) {
        None => {
            return Err( ParsingError::MissingMessageDeclarationError(ms_name) );
        },
        Some( ms_id ) => {
            got_ms_id = ms_id;
        }
    }
    // ***
    return Ok( Action::new(got_lf_id,got_ms_id,act_kind));
}
