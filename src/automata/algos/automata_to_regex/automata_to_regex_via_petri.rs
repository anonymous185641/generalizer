use std::{env, fs};
use std::process::Command;
use std::time::Duration;
use crate::automata::automaton::automaton::Automaton;
use crate::automata::communicating_automata::composition::benchmark::caut_benchmark_entry::PetrifyData;
use crate::automata::transition_systems::transition_systems_composition::ts_composition_error::TsCompositionError;
use crate::interactions::io::output::quick_drawing::draw_model;
use crate::interactions::syntax::interaction::Interaction;
use crate::petri_nets::petri_net::petri_net::PetriNet;
use crate::petri_nets::petri_net_io::petri_net_parsing::petri_parsing_interface::petri_nets_parse_file;
use crate::regex_inf::regex::regex::Regex;

impl Automaton{
    pub fn automaton_to_regex_via_petri_nets(aut: &Automaton,name:&str,files: &str,draw:bool) -> Result<((Regex<String>,Duration,usize)),String>{

        /////////////////////////////////////////////////////////////////////////////////
        /////////////////////////////////////////////////////////////////////////////////
        //Step 1: Transformation into Petri nets (Petrify)
        /////////////////////////////////////////////////////////////////////////////////
        /////////////////////////////////////////////////////////////////////////////////

        let ts_folder = format!("{}/transition_systems",&files);
        let _ = fs::create_dir_all(&ts_folder);

        let petri_folder = format!("{}/petri_net",&files);
        let _ = fs::create_dir_all(&petri_folder);
        //println!("HEre!!!!");
        //We draw the transition systems into .g files
        if draw{
            aut.to_g_file(name,&ts_folder);
            aut.draw(&name.to_string(),&ts_folder);
        }
        

        //Petrify call

        //We create a temporary folder to store the .g files and output files of Petrify

        let temp_petrify = "temp_petrify";
        //let _ = fs::remove_dir_all(temp_petrify);
        let _ = fs::create_dir_all(temp_petrify);
        //panic!();

        let exe_dir = env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .to_path_buf();

        let petrify = exe_dir.join("petrify");
        // Resolve absolute paths to avoid relative path parsing issues

        let input_file = std::path::Path::new(&ts_folder).join(format!("{}.g", name));
        let temp_input_file = std::path::Path::new(&temp_petrify).join(format!("{}.g", name));
        let _ = fs::copy(&input_file.clone(), &temp_input_file.clone());

        let output_file = std::path::Path::new(&petri_folder).join(format!("{}.out", name));
        let temp_output_file = std::path::Path::new(&temp_petrify).join(format!("{}.out", name));

        let start = std::time::Instant::now();
        let status = Command::new(&petrify)
            .arg(&temp_input_file)
            .arg("-dead")
            .arg("-o")
            .arg(&temp_output_file)
            //.current_dir(exe_dir) // Force working directory context
            .output();
        let petrify_time = start.elapsed();
        match status {
            Ok(output) => {
                if output.status.success() {
                    println!("{} successfully transformed into Petri net.", name);

                    //Copy the output file into the petri_nets folder
                    let _ = fs::copy(temp_output_file,output_file.clone());
                } else {
                    println!(
                        "{} transformation failed with exit code {:?}",
                        name,
                        output.status.code()
                    );

                    println!(
                        "stderr:\n{}",
                        String::from_utf8_lossy(&output.stderr)
                    );

                    println!(
                        "stdout:\n{}",
                        String::from_utf8_lossy(&output.stdout)
                    );

                    return Err("Petrify failure".to_string());
                }
            }
            Err(e) => {
                println!("{} transformation into Petri net failed.", name);
                println!("Error: {}", e);
                return Err(e.to_string());
            }
        }


        //panic!();
        println!("\n Transformation into Petri nets successful.\n");

        /////////////////////////////////////////////////////////////////////////////////
        /////////////////////////////////////////////////////////////////////////////////
        //Step 2: Transformation of Petri nets into Regular expressions
        /////////////////////////////////////////////////////////////////////////////////
        /////////////////////////////////////////////////////////////////////////////////
        println!("Beginning transformation of Petri nets into regular expressions.\n");



        let petri_name = format!("{}",name);
        let petri_file_path = format!("{}/petri_net/{}.out",files,petri_name);
        let derivation_process = format!("{}/{}_derivation_process", files, petri_name);
        let _ = fs::create_dir_all(&derivation_process);

        match petri_nets_parse_file::<Regex<String>>(&petri_file_path){
            Ok(mut pn)=>{
                println!("Petri net {}.out successfully parsed.",petri_name);

                //We eliminate duplicated transitions
                //For example the transition labeled a/1, or a/2... we become simply a
                //Maybe it is better to do this at parsing time??
                pn.eliminate_duplicated_transition_label();

                //Converting the Petri net into a regex
                match pn.petri_to_regex(&petri_name, Some(&derivation_process), true,draw){
                    Ok(regex)=>{
                        println!("Petri net {}.out successfully transformed into regular expression.",petri_name);

                        println!("REGEX: {}",regex.to_string());



                        return Ok((regex,petrify_time,pn.size()));
                    },
                    Err(e)=>{
                        println!("Error during the transformation of the Petri nets into regular expression.");
                        println!("Error: {}",e);

                        return Err(e.to_string());
                    }
                }

            },
            Err(e)=>{
                println!("Error during the parsing of the Petri nets.");
                println!("Error: {}",e);
                return Err("Error during the parsing of the Petri nets.".to_string());
            }
        }
    }

    pub fn automaton_to_petri_net(&self,files:&str)->Result<PetriNet<Regex<String>>,String>{

        todo!()
    }

}