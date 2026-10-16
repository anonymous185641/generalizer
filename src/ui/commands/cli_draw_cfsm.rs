use std::path::PathBuf;
use crate::automata::automata_io::automaton_parsing_g_files::automaton_interface_g::automaton_parse_g_file;
use crate::automata::cfsm::system_cfsm_io::parsing_system_cfsm::cfsm_interface::cfsm_parse_file;
use crate::petri_nets::petri_net_io::petri_net_parsing::petri_parsing_interface::petri_nets_parse_file;

pub fn cli_draw_cfsm(matches: &clap::ArgMatches) {
    let file_name = matches.value_of("file").unwrap();
    let with_gates = matches.is_present("gates");

    //Output folder preparation
    let output_dir = "draw_system_cfsm_output";
    let _ = std::fs::remove_dir_all(output_dir);
    let _ = std::fs::create_dir_all(output_dir);

    

    match cfsm_parse_file(file_name) {

        Ok(cfsm_system) => {

            println!(
                "\n=======================================================================\n\
                   The system of CFSM's file {} has been successfully parsed \n\
                    =======================================================================\n",file_name
            );
            let file_name_without_extension = if let Some(name) = PathBuf::from(file_name).file_stem().unwrap().to_str(){
                name.to_string()
            }
            else{
                file_name.to_string()
            };
            /*****/
            let cfsm_sysmtem_to_draw = if with_gates{
                println!("The system of CFSM will be drawn with gates with the same convention as to interactions models");
                cfsm_system.into_cfsm_with_gates()
            }
            else{
                cfsm_system.clone()
            };

            cfsm_sysmtem_to_draw.draw(&file_name_without_extension, &output_dir.to_string());
            println!("The output is in the following folder: {}",output_dir);

        }
        Err(e) => {
            println!("Error: {}", e);
        }

    }
}