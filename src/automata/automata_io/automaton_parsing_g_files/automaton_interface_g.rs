use std::fs;
use std::path::Path;
use crate::automata::automaton::automaton::Automaton;
use crate::automata::automata_io::automaton_parsing_g_files::automaton_string_parser_g::automaton_g_file_parse_string;

pub fn automaton_parse_g_file(file_path: &str) -> Result<Automaton,String> {

    let path = Path::new(file_path);



    /* File extension is no big deal
    let file_extension = path.extension().unwrap().to_str().unwrap();

    if file_extension != "g" && file_extension != "out" {
        return Err(format!("File extension is not .g or .out: {}", file_path));
    }
     */

    match fs::read_to_string(path) {
        Ok(unparsed_string)=>{

            let automaton = automaton_g_file_parse_string(&unparsed_string)?;

            Ok(automaton)
            
        },
        Err(_)=>{
            Err(format!("File error: {}", file_path))
        }
    }
}