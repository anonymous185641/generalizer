use std::path::PathBuf;
use crate::automata::automata_io::automaton_parsing_aut_files::automaton_interface_aut::automaton_parse_aut_file;
use crate::automata::automata_io::automaton_parsing_g_files::automaton_interface_g::automaton_parse_g_file;
use crate::automata::automaton::automaton::Automaton;
use crate::petri_nets::petri_net_io::petri_net_parsing::petri_parsing_interface::petri_nets_parse_file;
use crate::regex_inf::regex::regex::Regex;
use crate::ui::utils::print_file::{print_automaton_file, print_file_petri_to_regex};

pub fn cli_aut_to_regex(matches: &clap::ArgMatches){

    let file_name = matches.value_of("file").unwrap();

    let mut brzozowski = matches.is_present("brzozowski");
    let mut transitive_closure = matches.is_present("transitive_closure");
    let mut state_elimination = matches.is_present("state_elimination");
    let mut via_petrify = matches.is_present("via_petrify");
    let all_algorithms = matches.is_present("all_algorithms");

    if all_algorithms{
        brzozowski = true;
        transitive_closure = true;
        state_elimination = true;
        via_petrify = true;
    }
    if !brzozowski && !transitive_closure && !state_elimination && !via_petrify && !all_algorithms{
        brzozowski = true; //By default we use Brzozowski's algorithm
    }

    println!("Transformation of the automaton in file {} into a regular expression",file_name);

    match   automaton_parse_aut_file(file_name){
        Ok(mut aut)=>{
            //Printing the Petri net file
            print_automaton_file(file_name);

            //We try retrieve the name of the input file without the extension
            let file_name_without_extension = if let Some(name) = PathBuf::from(file_name).file_stem().unwrap().to_str(){
                name.to_string()
            }
            else{
                file_name.to_string()
            };

            //Application of the algorithms
            if brzozowski{

                println!(
                    "\n=======================================================================\n\
                    Derivation of regular expressions using Brzozowski's algorithm \n\
                    =======================================================================\n"
                );

                let regex = aut.to_regex_brzozowski();
               // println!("Regular expression RAW: {:?}\n",regex);

                println!("Regular expression with Brzozowski's algorithm: {:?}\n",regex.to_string());
            }

            if state_elimination{
                println!(
                    "\n=======================================================================\n\
                    Derivation of regular expressions using State elimination algorithm \n\
                    =======================================================================\n"
                );
                match aut.to_regex_with_state_elimination(){
                    Ok(regex)=>{



                        println!("Regular expression State Elimination: {:?}\n",regex.to_string());

                    },
                    Err(e)=>{
                        println!("Error: {}",e);
                    }
                }
            }

            if transitive_closure{

                println!(
                    "\n=======================================================================\n\
                    Derivation of regular expressions using Transitive closure algorithm \n\
                    =======================================================================\n"
                );

                match aut.to_regex_with_transitive_closure(){
                    Ok(regex)=>{
                        println!("Regular expression with transitive closure: {:?}\n",regex.to_string());
                    },
                    Err(e)=>{
                        println!("Error: {}",e);
                    }
                }
            }

            if via_petrify{
                println!(
                    "\n=======================================================================\n\
                    Derivation of regular expressions using Petrify \n\
                    =======================================================================\n"
                );

                let output_folder = "automata_to_regex_via_petri_nets_output";
                let _ = std::fs::remove_dir_all(output_folder);
                let _ = std::fs::create_dir_all(output_folder);
                match Automaton::automaton_to_regex_via_petri_nets(&aut,&file_name_without_extension,output_folder,true){
                    Ok((regex,_,_))=>{
                        println!("Regular expression with via Petrify: {:?}\n",regex.to_string());
                    },
                    Err(e)=>{
                        println!("Error: {}",e);
                    }
                }
            }


        }
        Err(e)=>{
            println!("Error: {}",e);
        }
    }

}