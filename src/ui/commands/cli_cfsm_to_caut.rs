use std::path::PathBuf;
use clap::ArgMatches;
use crate::automata::cfsm::system_cfsm_io::parsing_system_cfsm::cfsm_interface::cfsm_parse_file;
use crate::automata::communicating_automata::system_cautomata::cautomata::SystemCommunicatingAutomata;

pub fn cli_cfsm_to_caut(matches: &ArgMatches){

    let file_name = matches.value_of("file").unwrap();

    let target_folder = match matches.value_of("target_folder"){
        Some(s) => Some(s),
        _ => None
    };

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

            let system_caut = SystemCommunicatingAutomata::cfsm_to_caut(&cfsm_system);


            println!(
                "\n=======================================================================\n\
                   Translation to the .caut format successful  \n\
                    =======================================================================\n"
            );

            system_caut.to_caut_file(&file_name_without_extension,target_folder);

            println!(
                "\n=======================================================================\n\
                   The system of CAUT's file {} has been successfully created \n\
                    =======================================================================\n",file_name_without_extension
            );



        }
        Err(e) => {
            println!("Error: {}", e);
        }

    }

}