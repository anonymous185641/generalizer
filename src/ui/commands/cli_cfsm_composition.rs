use std::path::PathBuf;
use clap::ArgMatches;
use crate::automata::cfsm::system_cfsm_io::parsing_system_cfsm::cfsm_interface::cfsm_parse_file;

pub fn cli_cfsm_composition(matches: &ArgMatches){

    let file_name = matches.value_of("file").unwrap();

    //Output folder preparation
    let output_dir = "cfsm_composition_output";
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
            println!(
                "\n=======================================================================\n\
                   Beginning the composition of the system of CFSM's {}  \n\
                    =======================================================================\n",file_name
            );
            match cfsm_system.composition_to_global_interaction(&file_name_without_extension,&output_dir.to_string()){
                Ok(_) => {
                    println!("The composition of the system of CFSM's is successful");
                    println!("The output is in the following folder: {}",output_dir);
                },
                Err(e) => {
                    println!("The composition of the system of CFSM's is not successful");
                    println!("Error: {}", e);
                },
            }



        }
        Err(e) => {
            println!("Error: {}", e);
        }

    }


}
















