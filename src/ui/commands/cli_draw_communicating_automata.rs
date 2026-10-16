use std::path::PathBuf;
use crate::automata::automata_io::automaton_parsing_g_files::automaton_interface_g::automaton_parse_g_file;
use crate::automata::cfsm::system_cfsm_io::parsing_system_cfsm::cfsm_interface::cfsm_parse_file;
use crate::automata::communicating_automata::communicating_automata_io::parsing_system_caut::comm_aut_interface::communicating_automata_parse_file;
use crate::petri_nets::petri_net_io::petri_net_parsing::petri_parsing_interface::petri_nets_parse_file;

pub fn cli_draw_communicating_automata(matches: &clap::ArgMatches) {
    let file_name = matches.value_of("file").unwrap();
   

    //Output folder preparation
    let output_dir = "draw_system_concurrent_automata_output";
    let _ = std::fs::remove_dir_all(output_dir);
    let _ = std::fs::create_dir_all(output_dir);



    match communicating_automata_parse_file(file_name) {

        Ok(caut_system) => {

            println!(
                "\n=======================================================================\n\
                   The system of Concurrent Automata file {} has been successfully parsed \n\
                    =======================================================================\n",file_name
            );
            let file_name_without_extension = if let Some(name) = PathBuf::from(file_name).file_stem().unwrap().to_str(){
                name.to_string()
            }
            else{
                file_name.to_string()
            };
            /*****/


            caut_system.draw(&file_name_without_extension, &output_dir.to_string());
            println!("The output is in the following folder: {}",output_dir);

        }
        Err(e) => {
            println!("Error: {}", e);
        }

    }
}