
use pest::Parser;
use crate::automata::automaton::automaton::Automaton;
use crate::automata::automata_io::automaton_parsing_g_files::automaton_parser_g::{AutomatonGFileGrammarParser, Rule};
use crate::automata::automata_io::automaton_parsing_g_files::automaton_parser_g::Rule::{DCL_GRAPH_NAME, DCL_GRAPH_TYPE, DCL_MARKING, DCL_TRANSITIONS, TRANSITION, DCL_PATH, STATE_OR_TRANSITION};
use crate::automata::automata_io::automaton_parsing_g_files::util_types::{State, Trans};

pub fn automaton_g_file_parse_string(unparsed_string:&String) ->  Result<Automaton,String> {

    match AutomatonGFileGrammarParser::parse(Rule::PEST_FILE, unparsed_string) {
        Ok(ref mut got_pairs)=>{
            let mut transition_labels:Vec<Trans> = Vec::new();
            let mut state_labels:Vec<State> = Vec::new();
            let mut automaton = Automaton::new();

            for pair in got_pairs{
                if pair.as_rule() == DCL_GRAPH_NAME{
                    //Not important for now.
                }

                if pair.as_rule() == DCL_TRANSITIONS{
                    //Parsing of the transition labels
                    let pair_inner = pair.clone().into_inner();
                    for inner in pair_inner{
                        if inner.as_rule()==TRANSITION{
                            transition_labels.push(Trans(inner.as_str().to_string()));
                        }
                    }
                }
                if pair.as_rule()== DCL_GRAPH_TYPE{
                    //Not important for now.
                }
                if pair.as_rule() == DCL_PATH{
                    let pair_inner = pair.clone().into_inner();
                    let mut all_state_transitions_labels:Vec<String> = Vec::new();
                    for inner in pair_inner{
                        if inner.as_rule()==STATE_OR_TRANSITION{
                            all_state_transitions_labels.push(inner.as_str().to_string());
                        }
                    }
                    if all_state_transitions_labels.len() <3{
                        panic!("A path must have at least three elements.");
                    }

                    //Creation of new states
                    for label in &all_state_transitions_labels{
                        if !transition_labels.contains(&Trans(label.to_string())){
                            automaton.get_or_create_node(label);
                        }
                    }

                    //Creation of the transitions
                    let arcs: Vec<_> = all_state_transitions_labels.windows(3).step_by(2).collect();
                    for arc in arcs{
                        if arc.len()!=3{
                            panic!("A path must have exactly three elements. Bad arc: {:?}",arc);
                        }
                         let n1 = automaton.get_node(&arc[0]).unwrap();
                         let n2 = automaton.get_node(&arc[2]).unwrap();
                        automaton.graph.add_edge(n1,n2,arc[1].to_string());

                    }


                    //HERE YOU NEED TO HANDLE DE MESS OF .g ASSOCIATIONS
                   // println!("{:?}",all_state_transitions_labels);

                }

                if pair.as_rule() == DCL_MARKING{
                    //Parsing of the initial states
                    let marking_field = pair.clone().into_inner();

                    for marking_list in marking_field/*This has only one element*/{
                        let inner_markings = marking_list.clone().into_inner();

                        for marked_state in inner_markings{
                            let node_id = automaton.get_or_create_node(&marked_state.as_str().to_string());
                            automaton.initial_states.push(node_id);
                        }

                    }

                }

            }

            Ok(automaton)
        },
        Err(e)=>{
            return Err(e.to_string());
        }
    }
}