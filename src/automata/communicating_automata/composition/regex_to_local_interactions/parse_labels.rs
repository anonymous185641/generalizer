use crate::automata::communicating_automata::system_cautomata::cautomata::{CommunicatingAutomatonName, Gate};
use crate::automata::communicating_automata::system_cautomata::cautomata_actions::{CommunicatingAutomataAction, CommunicatingAutomataEmission, CommunicatingAutomataReception};

impl CommunicatingAutomataAction {
    pub fn parse_label(label:&str)->Result<CommunicatingAutomataAction,String>{
        /*
            The labels are of the form:
            "sendto(A,m)[g]" or "recvfrom(A,m)[g]" where A are automaton names, m is a string and g is a gate.

            In the case of sendto(A,m)[g], the message m is sent to the automaton A through the gate g.

            In the case of recvfrom(A,m)[g], the message m is received from the automaton A through the gate g.

            The gates g are optional.
        */
        fn parse_gate(s: &str) -> Result<Option<Gate>, String> {
            if s.is_empty() {
                return Ok(None);
            }

            if !s.starts_with('[') || !s.ends_with(']') {
                return Err(format!("Invalid gate specification: {}", s));
            }

            let gate = &s[1..s.len() - 1];

            if gate.is_empty() {
                Ok(None)
            } else {
                Ok(Some(Gate(gate.parse().unwrap())))
            }
        }

        fn parse_body(
            prefix: &str,
            label: &str,
        ) -> Result<(CommunicatingAutomatonName, String, Option<Gate>), String> {
            if !label.starts_with(prefix) {
                return Err(format!("Expected prefix '{}'", prefix));
            }

            let rest = &label[prefix.len()..];

            let closing = rest
                .find(')')
                .ok_or_else(|| "Missing ')'".to_string())?;

            let args = &rest[..closing];
            let gate_part = &rest[closing + 1..];

            let (automaton, message) = args
                .split_once(',')
                .ok_or_else(|| "Expected two arguments separated by ','".to_string())?;

            let gate = parse_gate(gate_part)?;

            Ok((CommunicatingAutomatonName(automaton.to_string()), message.to_string(), gate))
        }

        if label.starts_with("sendto(") {
            let (receiver, message, gate) = parse_body("sendto(", label)?;

            Ok(CommunicatingAutomataAction::Emission(
                CommunicatingAutomataEmission {
                    receiver,
                    message,
                    gate,
                },
            ))
        } else if label.starts_with("recvfrom(") {
            let (emitter, message, gate) = parse_body("recvfrom(", label)?;

            Ok(CommunicatingAutomataAction::Reception(
                CommunicatingAutomataReception {
                    emitter,
                    message,
                    gate,
                },
            ))
        } else {
            Err(format!("Unknown action label: {}", label))
        }

    }
}





















