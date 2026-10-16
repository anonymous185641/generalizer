use pest::Parser;
use crate::automata::automata_io::automaton_parsing_aut_files::automaton_parser_aut::{AutomatonAUTFileGrammarParser, Rule};
use crate::automata::automaton::automaton::Automaton;

pub fn automaton_aut_file_parse_string(unparsed_string:&String) ->  Result<Automaton,String> {
    match AutomatonAUTFileGrammarParser::parse(Rule::PEST_FILE, unparsed_string) {
        Ok(ref mut got_pairs)=>{
            let mut ret_automaton = Automaton::new();

            for pair in got_pairs{

                if pair.as_rule() == Rule::AUTOMATON{

                    let mut automaton = Automaton::new();
                    //let mut automaton_name_str = String::new();

                    let graph_innner = pair.clone().into_inner();
                    //println!("{}",graph_innner.as_str());
                    for graph_pair in graph_innner{

                        //We don't care about the name of the automaton
                        if graph_pair.as_rule()== Rule::DCL_GRAPH_TYPE{
                            //let name_inner = graph_pair.clone().into_inner().next().unwrap();

                          //  automaton_name_str = name_inner.as_str().to_string();
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
                                    transition_label.push_str(&path_pair.as_str().to_string());
                                }
                            }
                            //We update the automaton
                            let state1 = automaton.get_or_create_node(&states_labels[0]);
                            let state2 = automaton.get_or_create_node(&states_labels[1]);
                            automaton.create_edge_between(&state1, &state2, &transition_label);


                        }
                        if graph_pair.as_rule()==Rule::DCL_INITIAL_STATE{

                            let marking_field_inner = graph_pair.clone().into_inner().next().unwrap();//.unwrap().into_inner().next().unwrap();

                            if marking_field_inner.as_rule()==Rule::STATES{
                                let marking_inner_dbl = marking_field_inner.clone().into_inner();

                                for marking_list in marking_inner_dbl/*This has only one element*/{
                                    let inner_markings = marking_list.clone().into_inner();

                                    for marked_state in inner_markings{
                                        let node_id = automaton.get_or_create_node(&marked_state.as_str().to_string());
                                        automaton.initial_states.push(node_id);
                                    }
                                }
                            }
                        }

                        if graph_pair.as_rule()==Rule::DCL_FINAL_STATES{

                            let marking_field_inner = graph_pair.clone().into_inner().next().unwrap();//.into_inner().next().unwrap();

                            if marking_field_inner.as_rule()==Rule::STATES{
                                let marking_inner_dbl = marking_field_inner.clone().into_inner();

                                for marking_list in marking_inner_dbl/*This has only one element*/{
                                    let inner_markings = marking_list.clone().into_inner();

                                    for marked_state in inner_markings{
                                        let node_id = automaton.get_or_create_node(&marked_state.as_str().to_string());
                                        automaton.final_states.push(node_id);
                                    }
                                }
                            }
                        }
                    }

                    //We add the cfsm to the system
                    ret_automaton = automaton;
                }

            }

            Ok(ret_automaton)
        },
        Err(e)=>{
            return Err(e.to_string());
        }
    }
}