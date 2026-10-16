use std::fs;
use std::path::Path;
use crate::automata::cfsm::system_cfsm::system_cfsm::SystemCFSM;
use crate::automata::cfsm::system_cfsm_io::parsing_system_cfsm::cfsm_interface::cfsm_parse_file;

impl SystemCFSM{
    pub fn run_benchmarking_cfsm_composition(cfsm_folder: &str){

        let output_folder = "CFSM_COMPOSITION_BENCHMARKING";
        let _ = fs::remove_dir_all(output_folder);
        let _ = fs::create_dir(output_folder);


        for entry in fs::read_dir(Path::new(cfsm_folder)).unwrap() {
            let e = entry.unwrap();
            let entry_name = e.file_name().clone().into_string().unwrap();
            let entry_path =format!("{}/{}",cfsm_folder,&entry_name);

            //We create a folder for the entry
            let entry_output_path_str = format!("{}/{}", output_folder, entry_name);
            let _ = fs::create_dir(&entry_output_path_str);

            //Parsing of the entry
            match cfsm_parse_file(&entry_path) {

                Ok(cfsm_system) => {

                    if let Ok(interaction) = cfsm_system.composition_to_global_interaction(&entry_name,&entry_output_path_str){
                        println!("The composition of the CFSM {} is successful",&entry_name);
                    }else{
                        println!("Error during the composition of the CFSM {}\n\n",&entry_name);
                    }
                },
                Err(e) => {
                    println!("Error: {}", e);
                }
            }



        }
    }
}


























