use petgraph::visit::IntoEdges;
use crate::automata::cfsm::system_cfsm::system_cfsm::SystemCFSM;
use crate::global_counter::counter::fresh_number;
#[derive(Debug,Clone)]
pub struct GateCfsm {
    pub gate_id: usize,
    sender_id: usize,
    receiver_id: usize,
    message: String,
}
impl GateCfsm {
    pub fn new(gate_id: usize, sender_id: usize, receiver_id: usize, message: String) -> GateCfsm {
        GateCfsm {gate_id, sender_id, receiver_id, message}
    }

    pub fn fresh_gate(sender_id: usize, receiver_id: usize, message: String) -> GateCfsm {
        GateCfsm {gate_id: fresh_number(), sender_id, receiver_id, message}
    }
    pub fn get_gate_id(&self)->usize{
        self.gate_id
    }

    pub fn get_or_create_gate(gates:&mut Vec<GateCfsm>, sender_id: usize, receiver_id: usize, message: String) -> GateCfsm {
        for gate in gates.iter(){
            if gate.sender_id == sender_id
                && gate.receiver_id == receiver_id
                && gate.message == message{
                return gate.clone()
            }
        }

        let new_gate = GateCfsm::fresh_gate(sender_id, receiver_id, message);
        gates.push(new_gate.clone());
        new_gate
    }

}
impl SystemCFSM{
    pub fn into_cfsm_with_gates(&self) ->SystemCFSM{
        /*
            This function is used to preprocess the system of CFSM.

            It replaces transitions n!m by  l!m[g] where:
            - l is the lifeline of the CFSM, here it will be li where i is the id of the CFSM in self.machines vector.
            - m is the message of the transition. It is unchanged.
            - g is the gate, it identifies the compatibles emissions and  receptions  of the same messages accros CFSMs.
            for examle if in the machine of index 0 we have an action 1!m and in the machine of index 1 we have an action 0?m, we want that
            * in the machine 0,  1!m -> l0!m[g]
            * in the machine 1,   0?m -> l1?m[g]
            where g is the determine?
        */
        let mut new_system = SystemCFSM::new();
        let mut gates:Vec<GateCfsm> = Vec::new();

        for (machine_id,machine) in self.machines.iter().enumerate(){
            let mut new_machine = machine.clone();
            let edges = machine.graph.edge_indices();
            for edge in edges{

                new_machine.graph.remove_edge(edge);

                let (source_node,target_node) = machine.graph.edge_endpoints(edge).unwrap();
                let label = machine.graph.edge_weight(edge).unwrap();

                let (interlocutor_id,operator,message) = Self::parse_label(label);

               match operator{
                    '!'=>{
                      let gate =   GateCfsm::get_or_create_gate(&mut gates, machine_id, interlocutor_id, message.clone());
                        let new_label = format!("Emission_l{}_{}_{}",machine_id,message,gate.gate_id);
                        new_machine.graph.add_edge(source_node,target_node,new_label);
                    },
                    '?'=>{
                        let gate  = GateCfsm::get_or_create_gate(&mut gates, interlocutor_id, machine_id, message.clone());
                        let new_label = format!("Reception_l{}_{}_{}",machine_id,message,gate.gate_id);
                        new_machine.graph.add_edge(source_node,target_node,new_label);
                    },
                    _ =>{
                        panic!("Operator not supported. Must be ! or ?")
                    }
                }
                
                //new_machine.graph.update_edge(source_node,target_node,format!("l{}{}{}[{}]",machine_id,operator,message,gate.gate_id));
            }

            new_system.machines.push(new_machine);
        }

        new_system
    }

    pub fn parse_label(label: &str) -> (usize,char,String){
        let pos = label.find(['!', '?']).unwrap();

        let left = &label[..pos].parse::<usize>().ok().unwrap();
        let op = label.as_bytes()[pos] as char;
        let right = &label[pos + 1..];

        (left.clone(),op,right.to_string())
    }


}


















