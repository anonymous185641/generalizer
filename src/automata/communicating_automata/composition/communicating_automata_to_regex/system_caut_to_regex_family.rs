use std::fs;
use std::time::Duration;
use crate::automata::automaton::automaton::Automaton;
use crate::automata::cfsm::system_cfsm::system_cfsm::SystemCFSM;
use crate::automata::communicating_automata::composition::benchmark::caut_benchmark_entry::PetrifyData;
use crate::automata::communicating_automata::composition::caut_composition_error::CommunicatingAutomataCompositionError;
use crate::automata::communicating_automata::composition::communicating_automata_to_regex::automata_to_regex_algos::AutomataToRegexAlgo;
use crate::automata::communicating_automata::system_cautomata::cautomata::{CommunicatingAutomatonName, SystemCommunicatingAutomata};
use crate::global_counter::counter::fresh_number;
use crate::regex_inf::regex::regex::Regex;



impl SystemCommunicatingAutomata {
    
    pub fn to_regex_family(&self, algo: &AutomataToRegexAlgo,target_folder:&str,draw:bool)->Result<(Vec<(CommunicatingAutomatonName,Regex<String>)>,Option<PetrifyData>),CommunicatingAutomataCompositionError>{
        let mut regex_family = Vec::new();
        let mut petrify_data = None;
        
        let mut petrify_times = Vec::new();
        let mut petri_sizes = Vec::new();
        
        for (i, cautomaton) in self.automata.iter().enumerate(){

            let regex = match algo{
                AutomataToRegexAlgo::Brzozowski=> {
                    //Automaton::automaton_to_regex_brzozowski(&cautomaton.automaton)
                    cautomaton.automaton.to_regex_brzozowski()
                },
                AutomataToRegexAlgo::StateElimination=> {
                    cautomaton.automaton.to_regex_with_state_elimination().unwrap()
                },
                AutomataToRegexAlgo::TransitiveClosure=> {
                    cautomaton.automaton.to_regex_with_transitive_closure().unwrap()
                }
                AutomataToRegexAlgo::ViaPetriNets=> {
                   let files_folder = format!("{}/{}_derivation", target_folder, cautomaton.name.0);
                    let _ = fs::create_dir_all(files_folder.clone());

                    let aut = Self::preprocess_automaton_for_petrify(&cautomaton.automaton);
                    let (reg,petrify_time,petri_size) = match Automaton::automaton_to_regex_via_petri_nets(&aut,&cautomaton.name.0,files_folder.as_str(),draw){
                      Ok(tuple) => {
                          tuple
                      },
                        Err(e)=>{
                            return Err(CommunicatingAutomataCompositionError::RegexSyntesisError);
                        }
                    };
                    petrify_times.push(petrify_time);
                    petri_sizes.push(petri_size);
                    Self::translate_regex_because_of_petrify(&reg)
                }
            };
            
            regex_family.push((cautomaton.name.clone(),regex));
        }
        
        //We search for the min an max of the times and sizes
        if let AutomataToRegexAlgo::ViaPetriNets = algo{
            let min_petrify_time = *petrify_times.iter().min().unwrap();
            let max_petrify_time = *petrify_times.iter().max().unwrap();
            let min_petri_size = *petri_sizes.iter().min().unwrap();
            let max_petri_size = *petri_sizes.iter().max().unwrap();
            petrify_data = Some(PetrifyData{range_petrify_time:(min_petrify_time,max_petrify_time),range_petri_net_size: (min_petri_size,max_petri_size)});
        }
        
        Ok((regex_family,petrify_data))
       
    }

    pub fn preprocess_automaton_for_petrify(aut: &Automaton) ->Automaton{
        let mut new_automaton = aut.clone();
        for e in aut.graph.edge_indices(){
            //let (source,target) = aut.graph.edge_endpoints(e).unwrap();
            let weight = aut.graph.edge_weight(e).unwrap();
            let new_weight = Self::translate_label_into_petrify(&weight).unwrap();
            //new_automaton.graph.update_edge(source,target,Self::translate_label_into_petrify(weight).unwrap());//Dangerous unwrap
            new_automaton.graph[e] = new_weight;
        }
        //We insert empty transitions to mark final states
        new_automaton.introduce_empty_transition()
       // new_automaton
    }

    /*
    pub fn translate_label_into_petrify(input:&String) ->Option<String>{
        let (action, rest) = input.split_once('(')?;
        let (args, index_part) = rest.split_once(')')?;
        let (arg1, arg2) = args.split_once(',')?;
        let index = index_part.strip_prefix('[')?.strip_suffix(']')?;

        Some(format!("{action}_{arg1}_{arg2}_{index}"))
    }

     */
    pub fn translate_label_into_petrify(input: &String) -> Option<String> {
        let (action, rest) = input.split_once('(')?;
        let (args, index_part) = rest.split_once(')')?;
        let (arg1, arg2) = args.split_once(',')?;

        let index_part = index_part.trim();

        if index_part.is_empty() {
            Some(format!("{action}_{arg1}_{arg2}"))
        } else {
            let index = index_part
                .strip_prefix('[')?
                .strip_suffix(']')?;

            Some(format!("{action}_{arg1}_{arg2}_{index}"))
        }
    }

    pub fn translate_label_from_petrify(input: &String) -> Option<String> {
        let parts: Vec<&str> = input.split('_').collect();

        match parts.len() {
            3 => Some(format!("{}({},{})", parts[0], parts[1], parts[2])),
            4 => Some(format!(
                "{}({},{})[{}]",
                parts[0], parts[1], parts[2], parts[3]
            )),
            _ => None,
        }
    }
    /*
    pub fn translate_label_from_petrify(input:&String) ->Option<String>{
        let parts: Vec<&str> = input.split('_').collect();
        if parts.len() != 4 {
            return None;
        }

        Some(format!("{}({},{})[{}]", parts[0], parts[1], parts[2], parts[3]))
    }

     */

    pub fn translate_regex_because_of_petrify(regex: &Regex<String>)->Regex<String>{
        match regex.clone() {
            Regex::Epsilon => Regex::Epsilon,
            Regex::Empty => Regex::Empty,
            Regex::Literal(s) => Regex::Literal(Self::translate_label_from_petrify(&s).unwrap()),//Dangerous unwrap
            Regex::Concat(v)=>{
                let mut new_vec = Vec::new();
                for r in v{
                    new_vec.push(Self::translate_regex_because_of_petrify(&r));
                }
                Regex::Concat(new_vec)
            },
            Regex::Or(v)=>{
                let mut new_vec = Vec::new();
                for r in v{
                    new_vec.push(Self::translate_regex_because_of_petrify(&r));
                }
                Regex::Or(new_vec)
            },
            Regex::Int(v)=>{
                let mut new_vec = Vec::new();
                for r in v{
                    new_vec.push(Self::translate_regex_because_of_petrify(&r));
                }
                Regex::Int(new_vec)
            },
            Regex::Kleene(b)=>{
                let r = *b.clone();
                let r_prime = Self::translate_regex_because_of_petrify(&r);
                Regex::Kleene(Box::new(r_prime))
            },
            Regex::Plus(b)=>{
                let r = *b.clone();
                let r_prime = Self::translate_regex_because_of_petrify(&r);
                Regex::Plus(Box::new(r_prime))
            },
            Regex::Qm(b)=>{
                let r = *b.clone();
                let r_prime = Self::translate_regex_because_of_petrify(&r);
                Regex::Qm(Box::new(r_prime))
            },
            _ => panic!("Regex type not supported."),
        }
    }
}


impl Automaton{
    pub fn introduce_empty_transition(&self) -> Automaton{

        let mut new_automaton = self.clone();

        //Add empty transitions to final states

       

        for node in new_automaton.final_states.iter(){
            let new_node = new_automaton.graph.add_node(format!("qF{}",fresh_number()));
            new_automaton.graph.add_edge(node.clone(),new_node,"Epsilon".to_string());
        }

        new_automaton
    }


}