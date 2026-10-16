use pest::iterators::{Pair, Pairs};
use pest::Parser;
use crate::interactions::io::input::error::ParsingError;
use crate::interactions::io::input::hif::communication::parse_communication;
use crate::interactions::io::input::hif::parser::{HifParser, Rule};
use crate::interactions::syntax::general_context::GeneralContext;
use crate::interactions::syntax::interaction::{Interaction, LoopKind};


pub fn parse_hif_string(gen_ctx: &GeneralContext, hif_string: String) -> Result<Interaction,ParsingError>{
    match HifParser::parse(Rule::PEST_FILE,&hif_string){
        Ok(ref mut got_pair) => {
            let int_pair = got_pair.next().unwrap();
            match int_pair.as_rule() {
                Rule::SD_INTERACTION => {
                    return parse_interaction(gen_ctx, int_pair);
                },
                _ => {
                    panic!("what rule then ? : {:?}", int_pair.as_rule() );
                }
            }
        },
        Err(e)=> {
            return Err(ParsingError::MatchError(e.to_string()));
        }
    }
}


fn parse_interaction(gen_ctx : &GeneralContext, interaction_pair : Pair<Rule>) -> Result<Interaction,ParsingError> {
    let content_pair = interaction_pair.into_inner().next().unwrap();
    match content_pair.as_rule() {
        Rule::SD_EMPTY_INTERACTION => {
            return Ok( Interaction::Empty );
        },
        Rule::SD_COMMUNICATION => {
            return parse_communication(gen_ctx,&mut content_pair.into_inner());
        },
        Rule::SD_SEQ_INT => {
            match get_nary_sub_interactions_from_pair(gen_ctx, content_pair) {
                Err(e) => {
                    return Err(e);
                },
                Ok( mut sub_ints ) => {
                    return Ok( fold_interactions_in_binary_operator(&BinaryOperatorKind::Seq,&mut sub_ints) );
                }
            }
        },
        Rule::SD_ALT_INT => {
            match get_nary_sub_interactions_from_pair(gen_ctx, content_pair) {
                Err(e) => {
                    return Err(e);
                },
                Ok( mut sub_ints ) => {
                    return Ok( fold_interactions_in_binary_operator(&BinaryOperatorKind::Alt,&mut sub_ints) );
                }
            }
        },
        Rule::SD_PAR_INT => {
            match get_nary_sub_interactions_from_pair(gen_ctx, content_pair) {
                Err(e) => {
                    return Err(e);
                },
                Ok( mut sub_ints ) => {
                    return Ok( fold_interactions_in_binary_operator(&BinaryOperatorKind::Par,&mut sub_ints) );
                }
            }
        },
        Rule::SD_TENSOR_INT => {
            match get_nary_sub_interactions_from_pair(gen_ctx, content_pair) {
                Err(e) => {
                    return Err(e);
                },
                Ok( mut sub_ints ) => {
                    return Ok( fold_interactions_in_binary_operator(&BinaryOperatorKind::Tensor,&mut sub_ints) );
                }
            }
        },
        Rule::SD_COREG_INT => {
            let mut content = content_pair.into_inner();
            content.next(); // get rid of the operator name
            let coreg_lfs_pair = content.next().unwrap();
            match coreg_lfs_pair.as_rule() {
                Rule::LABEL_LIST => {
                    let mut target_lfs : Vec<usize> = Vec::new();
                    let mut inner_contents = coreg_lfs_pair.into_inner();
                    for tar_lf_pair in inner_contents {
                        let target_lf_name : String = tar_lf_pair.as_str().chars().filter(|c| !c.is_whitespace()).collect();
                        // ***
                        match gen_ctx.get_lf_id( &target_lf_name ) {
                            None => {
                                return Err( ParsingError::MissingLifelineDeclarationError( target_lf_name ) );
                            },
                            Some( tar_lf_id ) => {
                                if target_lfs.contains(&tar_lf_id) {
                                    return Err( ParsingError::OtherDefinitionError( format!("duplicate lifeline in co-region : {:}", target_lf_name) ) );
                                } else {
                                    target_lfs.push(tar_lf_id);
                                }
                            }
                        }
                    }
                    match get_nary_sub_interactions(gen_ctx, content) {
                        Err(e) => {
                            return Err(e);
                        },
                        Ok( mut sub_ints ) => {
                            return Ok( fold_interactions_in_binary_operator(&BinaryOperatorKind::CoReg(target_lfs),&mut sub_ints) );
                        }
                    }
                },
                _ => {
                    panic!("what rule then ? : {:?}", coreg_lfs_pair.as_rule() );
                }
            }
        },
        /* I dont understand why this code don't work
        Rule::SD_LOOPS_INT => {//Not sure

            let mut loop_content = content_pair.into_inner();
            match parse_interaction(gen_ctx,loop_content.next().unwrap()) {
                Err(e) => {
                    return Err(e);
                },
                Ok( sub_int ) => {
                    return Ok( Interaction::LoopS(Box::new(sub_int)) );
                }
            }


        },

         */
        Rule::SD_LOOP_INT => {
            let mut loop_content = content_pair.into_inner();
            let loop_kind_pair = loop_content.next().unwrap().into_inner().next().unwrap();
            match parse_interaction(gen_ctx,loop_content.next().unwrap()) {
                Err(e) => {
                    return Err(e);
                },
                Ok( sub_int ) => {
                    match loop_kind_pair.as_rule() {
                        Rule::SD_LOOP_KIND_S => {
                            return Ok( Interaction::Loop(LoopKind::SStrictSeq,Box::new(sub_int)) );
                        },
                        Rule::SD_LOOP_KIND_H => { //Please eliminate those cases
                            return Ok( Interaction::Loop(LoopKind::HHeadFirstWS,Box::new(sub_int)) );
                        },
                        Rule::SD_LOOP_KIND_W => { //Please eliminate those cases
                            return Ok( Interaction::Loop(LoopKind::WWeakSeq,Box::new(sub_int)) );
                        },
                        Rule::SD_LOOP_KIND_P => { //Please eliminate those cases
                            return Ok( Interaction::Loop(LoopKind::PInterleaving,Box::new(sub_int)) );
                        },
                        _ => {
                            unreachable!();
                        }
                    }
                }
            }
        },
        _ => {
            panic!("what rule then ? : {:?}", content_pair.as_rule());
        }
    }
}

fn get_nary_sub_interactions_from_pair(gen_ctx : &GeneralContext, sd_content_pair : Pair<Rule>) -> Result<Vec<Interaction>,ParsingError> {
    let mut content = sd_content_pair.into_inner();
    content.next(); // get rid of the operator name
    return get_nary_sub_interactions(gen_ctx, content);
}

fn get_nary_sub_interactions(gen_ctx : &GeneralContext, content : Pairs<Rule>) -> Result<Vec<Interaction>,ParsingError> {
    let mut sub_ints : Vec<Interaction> = Vec::new();
    for sub_interaction in content {
        match parse_interaction(gen_ctx,sub_interaction) {
            Err(e) => {
                return Err(e);
            },
            Ok( parsed_sub_int ) => {
                sub_ints.push( parsed_sub_int );
            }
        }
    }
    return Ok( sub_ints );
}

enum BinaryOperatorKind {
    Tensor,
    Seq,
    Par,
    Alt,
    CoReg(Vec<usize>),
}

fn fold_interactions_in_binary_operator(op_kind : &BinaryOperatorKind, sub_ints : &mut Vec<Interaction>) -> Interaction {
    assert!(sub_ints.len() > 0);
    if sub_ints.len() == 1 {
        return sub_ints.remove(0);
    } else {
        let first_int = sub_ints.remove(0);
        match op_kind {
            BinaryOperatorKind::Seq => {
                return Interaction::Seq( Box::new(first_int), Box::new(fold_interactions_in_binary_operator(op_kind,sub_ints)));
            },
            BinaryOperatorKind::Alt => {
                return Interaction::Alt( Box::new(first_int), Box::new(fold_interactions_in_binary_operator(op_kind,sub_ints)));
            },
            BinaryOperatorKind::Par => {
                return Interaction::Par( Box::new(first_int), Box::new(fold_interactions_in_binary_operator(op_kind,sub_ints)));
            },
            BinaryOperatorKind::Tensor => {
                return Interaction::Tensor( Box::new(first_int), Box::new(fold_interactions_in_binary_operator(op_kind,sub_ints)));
            },
            BinaryOperatorKind::CoReg(ref cr) => {
                return Interaction::CoReg( cr.clone(),Box::new(first_int), Box::new(fold_interactions_in_binary_operator(op_kind,sub_ints)));
            },
        }
    }
}
