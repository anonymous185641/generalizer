use crate::automata::cfsm::cfsm_composition::into_cfsm_with_gates::GateCfsm;
use crate::automata::cfsm::system_cfsm::system_cfsm::SystemCFSM;
use crate::automata::communicating_automata::system_cautomata::cautomata::{CommunicatingAutomaton, CommunicatingAutomatonMessage, CommunicatingAutomatonName, Gate, SystemCommunicatingAutomata};
use crate::automata::communicating_automata::system_cautomata::cautomata_context::SystemCommunicatingAutomataContext;

impl SystemCommunicatingAutomata{

    pub fn cfsm_to_caut(system_cfsm: &SystemCFSM)->SystemCommunicatingAutomata{
        let mut system_caut = SystemCommunicatingAutomata::new();
        let mut system_caut_context = SystemCommunicatingAutomataContext::new();

        let mut gates:Vec<GateCfsm> = Vec::new();

        for (machine_id,machine) in system_cfsm.machines.iter().enumerate(){
            let mut new_machine = machine.clone();
            let name = format!("A{}",machine_id);

            let edges = machine.graph.edge_indices();
            for edge in edges{

                new_machine.graph.remove_edge(edge);

                let (source_node,target_node) = machine.graph.edge_endpoints(edge).unwrap();
                let label = machine.graph.edge_weight(edge).unwrap();

                let (interlocutor_id,operator,message) = SystemCFSM::parse_label(label);
                let interlocutor_name = format!("A{}",interlocutor_id);

                //Adding infos to the context
                system_caut_context.add_automaton_name(CommunicatingAutomatonName(name.clone()));
                system_caut_context.add_message(CommunicatingAutomatonMessage(message.clone()));

                match operator{
                    '!'=>{
                        let gate =   GateCfsm::get_or_create_gate(&mut gates, machine_id, interlocutor_id, message.clone());
                        let new_label = format!("sendto({},{})[{}]",interlocutor_name,message,gate.gate_id);
                        new_machine.graph.add_edge(source_node,target_node,new_label);
                    },
                    '?'=>{
                        let gate  = GateCfsm::get_or_create_gate(&mut gates, interlocutor_id, machine_id, message.clone());
                        let new_label = format!("recvfrom({},{})[{}]",interlocutor_name,message,gate.gate_id);
                        new_machine.graph.add_edge(source_node,target_node,new_label);
                    },
                    _ =>{
                        panic!("Operator not supported. Must be ! or ?")
                    }
                }

            }
            /////////////////
            //Final states
            ////////////////

            //First we add the initial state to the final states
            new_machine.final_states.push(new_machine.initial_states[0].clone());

            //Then we add the nodes with no outgoing edges to the final states
            for node in new_machine.graph.node_indices(){
                if new_machine.graph.neighbors_directed(node,petgraph::Direction::Outgoing).count() == 0{
                    new_machine.final_states.push(node);
                }
            }

            //Adding the new machine to the system
            system_caut.automata.push(CommunicatingAutomaton::new(new_machine,CommunicatingAutomatonName(name)));
        }

        //Adding gates to the context
        for gate in gates.iter(){
            system_caut_context.add_gate(Gate(gate.gate_id));
        }

        //Sorting and deduplication
        system_caut_context.automata_names.sort();
        system_caut_context.automata_names.dedup();

        system_caut_context.messages.sort();
        system_caut_context.messages.dedup();

        system_caut.context = system_caut_context;


        system_caut
    }
}





































