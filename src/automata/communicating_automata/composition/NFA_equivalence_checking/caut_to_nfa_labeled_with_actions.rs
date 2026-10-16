use crate::automata::communicating_automata::system_cautomata::cautomata::CommunicatingAutomaton;
use crate::automata::communicating_automata::system_cautomata::cautomata_actions::CommunicatingAutomataAction;
use crate::interactions::syntax::action::{Action, ActionType};
use crate::interactions::syntax::general_context::GeneralContext;
use crate::interactions::syntax::interaction::Interaction;

impl CommunicatingAutomaton{
    pub fn label_conversion(&self,label: &String,gen_ctx: &GeneralContext,alphabet: &Vec<Action>)->Option<usize>{
        let caut_action = match CommunicatingAutomataAction::parse_label(label){
            Ok(action)=>action,
            Err(e)=>return None,
        };
        
        let action_int =  match caut_action{
            CommunicatingAutomataAction::Emission(e)=>{
                let msg = e.message.clone();

                let gate = if let Some(gate) = e.gate{
                    Some(gate.0)
                }else{
                    None
                };
                let act = Action::new(gen_ctx.get_lf_id(&self.name.0).unwrap(),gen_ctx.get_ms_id(&msg).unwrap(),ActionType::Emission);
                act
            },
            CommunicatingAutomataAction::Reception(r)=>{
                let msg = r.message.clone();

                let gate = if let Some(gate) = r.gate{
                    Some(gate.0)
                }else{
                    None
                };
                let act = Action::new(gen_ctx.get_lf_id(&self.name.0).unwrap(),gen_ctx.get_ms_id(&msg).unwrap(),ActionType::Reception);
                act
            }
        };
        
        for (i,act) in alphabet.iter().enumerate(){
            if *act == action_int{
                return Some(i);
            } 
        }  
        
        None
    }
}













