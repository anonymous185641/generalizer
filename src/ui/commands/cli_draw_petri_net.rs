use std::path::PathBuf;
use crate::petri_nets::petri_net_io::petri_net_parsing::petri_parsing_interface::petri_nets_parse_file;
use crate::regex_inf::regex::regex::Regex;

pub fn cli_draw_petri_net(matches: &clap::ArgMatches) {
    let file_name = matches.value_of("file").unwrap();

    //Output folder preparation
    let output_dir = "draw_petri_net_output";
    let _ = std::fs::remove_dir_all(output_dir);
    let _ = std::fs::create_dir_all(output_dir);


    match   petri_nets_parse_file::<Regex<String>> (file_name) {
        Ok(pn) => {
            
            println!(
                "\n=======================================================================\n\
                    Petri net file {} has been successfully parsed \n\
                    =======================================================================\n",file_name
            );
            let file_name_without_extension = if let Some(name) = PathBuf::from(file_name).file_stem().unwrap().to_str(){
                name.to_string()
            }
            else{
                file_name.to_string()
            };
            
            pn.draw(&file_name_without_extension,&output_dir.to_string());
            println!("The output is in the following folder: {}",output_dir);
            
        },
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}