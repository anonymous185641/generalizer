use std::{fmt, fs};
use crate::interactions::syntax::action::{Action, ActionType};
use crate::interactions::syntax::error::CoreError;
use crate::interactions::syntax::general_context::GeneralContext;
use crate::interactions::syntax::interaction::Interaction;
use crate::terms::term::Term;

#[derive(Debug,Clone,PartialEq,Eq,Hash)]
pub struct ValuePassing {
    pub lf_emission_id : usize,
    pub lf_reception_id : usize,
    pub msg_id : usize
}
pub type SynchronousTrace = Vec<ValuePassing>;





impl ValuePassing{
    pub fn new(emission : Action, reception : Action) -> Self
    {
        Self{lf_emission_id: emission.lf_id,lf_reception_id: reception.lf_id, msg_id: reception.ms_id}
    }
    
    pub fn from_pair(pair : &(Action,Action)) -> Result<Self,String>{
        match (pair.0.action_type.clone(),pair.1.action_type.clone()){
            (ActionType::Emission,ActionType::Reception) => Ok(Self::new(pair.0.clone(),pair.1.clone())),
            (ActionType::Reception,ActionType::Emission) => Ok(Self::new(pair.1.clone(),pair.0.clone())),
            _ => Err(String::from("Invalid pair of actions"))
        }
    }

    pub fn to_interaction(&self)-> Interaction{
        let a1 = Action::new(self.lf_emission_id,self.msg_id,ActionType::Emission);
        let a2 = Action::new(self.lf_reception_id,self.msg_id,ActionType::Reception);
        Interaction::Vp(a1,a2)
    }

    pub fn parse_vp(text: &str) -> Result<Self, String> {
        // Check prefix and suffix
        if !text.starts_with("vp(l") || !text.ends_with(")") {
            return Err("Invalid format".to_string());
        }

        // Strip `vp(` prefix and `)` suffix
        let inner = &text[3..text.len() - 1]; // removes "vp(" and ")"
        // inner should now be: lX!mY?lZ

        // Split at '!'
        let (left, rest) = inner
            .split_once('!')
            .ok_or("Missing '!' separator")?;

        // left should be lX
        if !left.starts_with('l') {
            return Err("Invalid emission part".to_string());
        }
        let lf_emission_id = left[1..]
            .parse::<usize>()
            .map_err(|_| "Invalid emission id")?;

        // Split rest at '?'
        let (middle, right) = rest
            .split_once('?')
            .ok_or("Missing '?' separator")?;

        // middle should be mY
        if !middle.starts_with('m') {
            return Err("Invalid message part".to_string());
        }
        let msg_id = middle[1..]
            .parse::<usize>()
            .map_err(|_| "Invalid message id")?;

        // right should be lZ
        if !right.starts_with('l') {
            return Err("Invalid reception part".to_string());
        }
        let lf_reception_id = right[1..]
            .parse::<usize>()
            .map_err(|_| "Invalid reception id")?;

        Ok(ValuePassing {
            lf_emission_id,
            lf_reception_id,
            msg_id,
        })
    }

    pub fn to_text(&self,gen_ctx:&GeneralContext) -> Result<String,CoreError>{
        let lf_emission_name = gen_ctx.get_lf_name(self.lf_emission_id)?;
        let lf_reception_name = gen_ctx.get_lf_name(self.lf_reception_id)?;
        let msg_name = gen_ctx.get_ms_name(self.msg_id)?;
        Ok(format!("vp({}!{}?{})",lf_emission_name,msg_name,lf_reception_name))
    }
}

impl Interaction{
    pub fn sync_trace_to_text(trace:&SynchronousTrace,gen_ctx:&GeneralContext) -> Result<String,CoreError>{
        let mut buffer = String::new();
        for vp in trace.iter(){
            buffer.push_str(&vp.to_text(gen_ctx)?);
            buffer.push_str("\n");
        }
        Ok(buffer)
    }

    pub fn sync_trace_to_file(trace:&SynchronousTrace,gen_ctx:&GeneralContext,file_name:&str) -> Result<(),CoreError>{
        let _ = fs::write(file_name,Self::sync_trace_to_text(trace,gen_ctx)?);
        Ok(())
    }
}



impl fmt::Display for ValuePassing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let buffer = format!("vp(l{}!m{}?l{})",self.lf_emission_id,self.msg_id,self.lf_reception_id);

        write!(f, "{}", buffer)

    }
}