use crate::automata::communicating_automata::system_cautomata::cautomata_actions::CommunicatingAutomataValuePassing;
use crate::interactions::syntax::action::{Action, ActionType};
use crate::interactions::syntax::general_context::GeneralContext;
use crate::interactions::syntax::interaction::Interaction;

impl CommunicatingAutomataValuePassing{
    pub fn parse_value_passing(general_context: &GeneralContext,label: &String)->Result<Interaction,String>{
        let (emitter, rest) = label
            .split_once("--")
            .ok_or("Missing '--'")?;

        let (message, rest) = rest
            .split_once("->")
            .ok_or("Missing '->'")?;

        let (receiver, gate) = if let Some((receiver, gate)) = rest.split_once('[') {
            let gate = gate
                .strip_suffix(']')
                .ok_or("Missing closing ']'")?
                .parse::<usize>()
                .map_err(|_| "Invalid gate number")?;

            (receiver, Some(gate))
        } else {
            (rest, None)
        };
        
        let emitter_id = general_context.get_lf_id(emitter).unwrap();
        let receiver_id = general_context.get_lf_id(receiver).unwrap();
        let message_id = general_context.get_ms_id(message).unwrap();
        
        let emission_action = Action::new_with_id_raw(emitter_id,message_id,ActionType::Emission,gate);
        let reception_action = Action::new_with_id_raw(receiver_id,message_id,ActionType::Reception,gate);
        
        Ok(Interaction::Vp(emission_action,reception_action))
    }
}