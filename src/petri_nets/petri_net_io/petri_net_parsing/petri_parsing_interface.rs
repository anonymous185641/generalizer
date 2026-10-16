use std::fmt::Debug;
use std::fs;
use std::hash::Hash;
use std::path::Path;
use crate::automata::automaton::automaton::Automaton;
use crate::automata::automata_io::automaton_parsing_g_files::automaton_string_parser_g::automaton_g_file_parse_string;
use crate::petri_nets::petri_net::petri_net::PetriNet;
use crate::petri_nets::petri_net::place_transition::PetriNetTransitionLabelBound;
use crate::petri_nets::petri_net_io::petri_net_parsing::petri_nets_string_parser::{petri_nets_parse_string, LiteralFromString};

pub fn petri_nets_parse_file<T: PetriNetTransitionLabelBound>(file_path: &str) -> Result<PetriNet<T>,String> {

    let path = Path::new(file_path);



    /* File extension is no big deal
    let file_extension = path.extension().unwrap().to_str().unwrap();

    if file_extension != "g" && file_extension != "out" {
        return Err(format!("File extension is not .g or .out: {}", file_path));
    }
     */

    match fs::read_to_string(path) {
        Ok(unparsed_string)=>{

            let mut pn = petri_nets_parse_string(&unparsed_string)?;
            pn.repair_isolated_transitions();
            Ok(pn)

        },
        Err(_)=>{
            Err(format!("File error: {}", file_path))
        }
    }
}