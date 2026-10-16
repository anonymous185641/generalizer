use std::fmt::Display;
use crate::automata::communicating_automata::system_cautomata::cautomata::{CommunicatingAutomatonName, Gate};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CommunicatingAutomataEmission {
    pub receiver: CommunicatingAutomatonName,
    pub message: String,
    pub gate: Option<Gate>
}


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CommunicatingAutomataReception {
    pub emitter: CommunicatingAutomatonName,
    pub message: String,
    pub gate: Option<Gate>
}


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum CommunicatingAutomataAction {
    Emission(CommunicatingAutomataEmission),
    Reception(CommunicatingAutomataReception),
}

impl CommunicatingAutomataAction{

    pub fn are_complementary(a: &CommunicatingAutomataAction,b: &CommunicatingAutomataAction)->bool{
        match (a,b){
            (CommunicatingAutomataAction::Emission(e),CommunicatingAutomataAction::Reception(r))=>{
                if let (Some(gate_a),Some(gate_b)) = (e.gate.clone(),r.gate.clone()){
                    return gate_a == gate_b && e.message == r.message && e.receiver != r.emitter;
                }else{
                    return false;
                }
            },
            (CommunicatingAutomataAction::Reception(r),CommunicatingAutomataAction::Emission(e))=>{
                if let (Some(gate_a),Some(gate_b)) = (e.gate.clone(),r.gate.clone()){
                    return gate_a == gate_b && e.message == r.message && e.receiver != r.emitter;
                }else{
                    return false;
                }
            },
            _ => false
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CommunicatingAutomataValuePassing{
    pub emission: CommunicatingAutomataEmission,
    pub reception: CommunicatingAutomataReception
}


impl std::fmt::Display for CommunicatingAutomataValuePassing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       if let (Some(gate_a),Some(gate_b)) = (self.emission.gate.clone(),self.reception.gate.clone()){
           write!(f, "{}--{}->{}[{}]", self.reception.emitter.0,self.reception.message,self.emission.receiver.0,gate_a.0)
       }
        else{
            write!(f, "{}--{}->{}", self.reception.emitter.0,self.reception.message,self.emission.receiver.0)
        }
        
    }
}