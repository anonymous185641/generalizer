use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use crate::petri_nets::petri_net_io::petri_net_parsing::petri_parsing_interface::petri_nets_parse_file;
use crate::petri_nets::petri_nets_to_regex::step2_alt_int_separation::alt_int_separation;
use crate::regex_inf::regex::regex::Regex;
use crate::ui::utils::print_file::print_file_petri_to_regex;

pub fn cli_petri_net_to_regex(matches: &clap::ArgMatches){

    let file_name = matches.value_of("file").unwrap();




    println!("Transformation of the Petri net in file {} into a regular expression",file_name);



    match   petri_nets_parse_file::<Regex<String>>(file_name){
        Ok(mut pn)=>{
           //Printing the Petri net file
            print_file_petri_to_regex(file_name);

            //We try retrieve the name of the input file without the extension
            let file_name_without_extension = if let Some(name) = PathBuf::from(file_name).file_stem().unwrap().to_str(){
                name.to_string()
            }
            else{
                file_name.to_string()
            };
            pn.eliminate_duplicated_transition_label();
            match pn.petri_to_regex(&file_name_without_extension,None,true,true){
                Ok(regex)=>{
                       println!("Regular expression: {:?}",regex);

                },
                Err(e)=>{
                    println!("Error: {}",e);
                }
            }

        }
        Err(e)=>{
            println!("Error: {}",e);
        }
    }

}