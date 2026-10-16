use std::fs;
use std::path::Path;
use crate::automata::automata_io::automaton_parsing_g_files::automaton_string_parser_g::automaton_g_file_parse_string;
use crate::automata::automaton::automaton::Automaton;
use crate::automata::cfsm::system_cfsm_io::parsing_system_cfsm::cfsm_string_parser::cfsm_parse_string;
use crate::automata::cfsm::system_cfsm::system_cfsm::SystemCFSM;
use crate::automata::communicating_automata::communicating_automata_io::parsing_system_caut::comm_aut_string_parser::communicating_aut_parse_string;
use crate::automata::communicating_automata::system_cautomata::cautomata::SystemCommunicatingAutomata;

pub fn communicating_automata_parse_file(file_path: &str) -> Result<SystemCommunicatingAutomata,String> {

    let path = Path::new(file_path);

    let file_name = path.file_name().unwrap().to_str().unwrap();

    /* File extension is no big deal
    let file_extension = path.extension().unwrap().to_str().unwrap();

    if file_extension != "g" && file_extension != "out" {
        return Err(format!("File extension is not .g or .out: {}", file_path));
    }
     */

    match fs::read_to_string(path) {
        Ok(unparsed_string)=>{

            let mut parsed_caut = communicating_aut_parse_string(&unparsed_string)?;
            
            parsed_caut.system_name = file_name.to_string();
            
            Ok(parsed_caut)

        },
        Err(_)=>{
            Err(format!("File error: {}", file_path))
        }
    }
}