use std::path::PathBuf;
use clap::ArgMatches;
use crate::automata::communicating_automata::communicating_automata_io::parsing_system_caut::comm_aut_interface::communicating_automata_parse_file;
use crate::automata::communicating_automata::composition::communicating_automata_to_regex::automata_to_regex_algos::AutomataToRegexAlgo;

pub fn cli_synchronous_communicating_automata_composition(matches: &ArgMatches) {
    let file_name = matches.value_of("file").unwrap();

    //Parameters
    let mut brzozowski = matches.is_present("brzozowski");
    let mut transitive_closure = matches.is_present("transitive_closure");
    let mut state_elimination = matches.is_present("state_elimination");
    let mut via_petrify = matches.is_present("via_petrify");
    let all_algorithms = matches.is_present("all_algorithms");

    if all_algorithms {
        brzozowski = true;
        transitive_closure = true;
        state_elimination = true;
        via_petrify = true;
    }
    if !brzozowski && !transitive_closure && !state_elimination && !via_petrify && !all_algorithms {
        brzozowski = true; //By default we use the method via Petrify
    }

    let mut algos = vec![];
    if brzozowski {
        algos.push(AutomataToRegexAlgo::Brzozowski);
    }
    if transitive_closure {
        algos.push(AutomataToRegexAlgo::TransitiveClosure);
    }
    if state_elimination {
        algos.push(AutomataToRegexAlgo::StateElimination);
    }
    if via_petrify {
        algos.push(AutomataToRegexAlgo::ViaPetriNets);
    }

    //Output folder preparation
    let output_dir = "Composition_From_Sync_Product_output";
    let _ = std::fs::remove_dir_all(output_dir);
    let _ = std::fs::create_dir_all(output_dir);

    match communicating_automata_parse_file(file_name) {

        Ok(communicating_automata_system) => {

            println!(
                "\n=======================================================================\n\
                   The system of Communicating automata file {} has been successfully parsed \n\
                    =======================================================================\n",file_name
            );
            let file_name_without_extension = if let Some(name) = PathBuf::from(file_name).file_stem().unwrap().to_str(){
                name.to_string()
            }
            else{
                file_name.to_string()
            };
            println!(
                "\n=======================================================================\n\
                   Beginning the synchronous composition of the system of communicating automata {}  \n\
                    =======================================================================\n",file_name
            );


            for algo in algos{
                println!("Composing the system of communicating automata with the {} algorithm",algo.get_algo_name());
                match communicating_automata_system.sync_composition_system_communicating_automata(&algo, &file_name_without_extension, &output_dir.to_string()){
                    Ok(_) => {
                        println!("The composition of the system of communicating automata is successful");
                        println!("The output is in the following folder: {}",output_dir);
                    },
                    Err(e) => {
                        println!("The composition of the system of communicating automata is not successful");
                        println!("Error: {}", e);
                    },
                }
            }
        }
        Err(e) => {
            println!("Error: {}", e);
        }

    }
}