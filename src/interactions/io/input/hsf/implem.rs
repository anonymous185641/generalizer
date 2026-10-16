use pest::iterators::Pair;
use pest::Parser;
use crate::interactions::io::input::error::ParsingError;
use crate::interactions::io::input::hsf::parser::{HsfParser, Rule};
use crate::interactions::syntax::general_context::GeneralContext;



pub fn parse_hsf_string(hsf_string: String) -> Result<GeneralContext,ParsingError>{
    match HsfParser::parse(Rule::HSF_PEST_FILE,&hsf_string){
        Ok(ref mut got_pair) => {
            let sig_pair = got_pair.next().unwrap();
            
            match sig_pair.as_rule(){
                Rule::SIGNATURE => {
                    return parse_signature(sig_pair);
                },
                _ => {
                    panic!("what rule then ? : {:?}", sig_pair.as_rule() );
                }
            }
        },
        Err(e) => {
           return Err(ParsingError::MatchError(e.to_string())); 
        }
    }
}

fn parse_signature(signature_pair: Pair<Rule>) -> Result<GeneralContext,ParsingError>{
    
    let mut multiple_section_message : bool = false;
    let mut multiple_section_lifeline : bool = false;
    
    let mut contents = signature_pair.into_inner();
    
    let mut gen_ctx = GeneralContext::new();
    
    while let Some(current_pair) = contents.next(){
        match current_pair.as_rule(){
            Rule::SIG_MS_DECL => {
                if multiple_section_message{
                    return Err(ParsingError::HsfSetupError("several '@message' sections declared".to_string()));
                }
                multiple_section_message = true;
                parse_message_decl(current_pair,&mut gen_ctx);
            },
            Rule::SIG_LF_DECL => {
                if multiple_section_lifeline{
                    return Err(ParsingError::HsfSetupError("several '@lifeline' sections declared".to_string()));
                }
                multiple_section_lifeline = true;
                parse_lifeline_decl(current_pair,&mut gen_ctx);
            },
            _ => {
                panic!("what rule then ? : {:?}", current_pair.as_rule() );
            }
        }
    }
    
     Ok(gen_ctx)
}

fn parse_message_decl(msg_decl_pair: Pair<Rule>,gen_ctx: &mut GeneralContext){
    for msg_pair in msg_decl_pair.into_inner(){
        let ms_name: String = msg_pair.as_str().chars().filter(|c| !c.is_whitespace()).collect();
        gen_ctx.add_msg(ms_name);
    }
}

fn parse_lifeline_decl(lf_decl_pair: Pair<Rule>,gen_ctx: &mut GeneralContext){
    for lf_pair in lf_decl_pair.into_inner(){
        let lf_name: String = lf_pair.as_str().chars().filter(|c| !c.is_whitespace()).collect();
        gen_ctx.add_lf(lf_name);
    }
}