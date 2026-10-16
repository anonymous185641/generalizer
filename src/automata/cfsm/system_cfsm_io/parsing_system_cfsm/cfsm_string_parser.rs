use pest::Parser;
use crate::automata::automaton::automaton::Automaton;
use crate::automata::cfsm::system_cfsm_io::parsing_system_cfsm::cfsm_parser::{CFSMGrammarParser, Rule};
use crate::automata::cfsm::system_cfsm::system_cfsm::SystemCFSM;

pub fn cfsm_parse_string(unparsed_string:&String) ->  Result<SystemCFSM,String> {

    match CFSMGrammarParser::parse(Rule::PEST_FILE, unparsed_string) {
        Ok(ref mut got_pairs)=>{
            let mut system_cfsm = SystemCFSM::new();

            for pair in got_pairs{

                if pair.as_rule() == Rule::GRAPH{

                    let mut  cfsm = Automaton::new();

                    let graph_innner = pair.clone().into_inner();
                    //println!("{}",graph_innner.as_str());
                    for graph_pair in graph_innner{
                        if graph_pair.as_rule() == Rule::DCL_GRAPH_NAME{
                            //Not important for now.
                        }

                        if graph_pair.as_rule()== Rule::DCL_GRAPH_TYPE{
                            //Not important for now.

                        }

                        if graph_pair.as_rule()==Rule::DCL_PATH{

                            let path_inner = graph_pair.clone().into_inner();

                            let mut states_labels:Vec<String> = Vec::new();
                            let mut transition_label = String::new();
                            for path_pair in path_inner{
                                if path_pair.as_rule()==Rule::STATE_NAME{
                                    states_labels.push(path_pair.as_str().to_string());
                                }
                                if path_pair.as_rule()==Rule::TRANSITION{
                                    //We retrieve a transition 'n ! m' as 'n!m'
                                    for trans_pair in path_pair.clone().into_inner(){
                                        if trans_pair.as_rule()==Rule::INTERLOCUTOR{
                                            transition_label.push_str(&trans_pair.as_str().to_string());
                                        }
                                        if trans_pair.as_rule()==Rule::COMMUNICATION_TYPE{
                                            transition_label.push_str(&trans_pair.as_str().to_string());
                                        }
                                        if trans_pair.as_rule()==Rule::MESSAGE{
                                            transition_label.push_str(&trans_pair.as_str().to_string());
                                        }
                                    }
                                    transition_label = transition_label.replace(' ',"");
                                }
                            }
                            //We update the automaton
                            let state1 = cfsm.get_or_create_node(&states_labels[0]);
                            let state2 = cfsm.get_or_create_node(&states_labels[1]);
                            cfsm.create_edge_between(&state1, &state2, &transition_label);


                        }
                        if graph_pair.as_rule()==Rule::DCL_MARKING{
                            
                            let marking_field_inner = graph_pair.clone().into_inner().next().unwrap().into_inner().next().unwrap();
                            
                            if marking_field_inner.as_rule()==Rule::MARKING_MANY{
                                let marking_inner_dbl = marking_field_inner.clone().into_inner();

                                for marking_list in marking_inner_dbl/*This has only one element*/{
                                    let inner_markings = marking_list.clone().into_inner();

                                    for marked_state in inner_markings{
                                        let node_id = cfsm.get_or_create_node(&marked_state.as_str().to_string());
                                        cfsm.initial_states.push(node_id);
                                    }
                                }
                            }
                            
                            if marking_field_inner.as_rule()==Rule::MARKING_SINGLE{
                                let marking_inner_dbl = marking_field_inner.clone().into_inner();
                                let node_id = cfsm.get_or_create_node(&marking_inner_dbl.as_str().to_string());
                                cfsm.initial_states.push(node_id);
                            }
                        }
                    }

                    //We add the cfsm to the system
                    system_cfsm.machines.push(cfsm);
                }

            }

            Ok(system_cfsm)
        },
        Err(e)=>{
            return Err(e.to_string());
        }
    }
}