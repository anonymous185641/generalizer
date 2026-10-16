use std::path::PathBuf;
use clap::ArgMatches;
use crate::automata::communicating_automata::communicating_automata_io::parsing_system_caut::comm_aut_interface::communicating_automata_parse_file;
use crate::automata::communicating_automata::product::sync_product::sync_product::SyncProduct;

pub fn cli_sync_product(matches: &ArgMatches) {
    let file_name = matches.value_of("file").unwrap();

    //Output folder preparation
    let output_dir = "synchronous_product_output";
    let _ = std::fs::remove_dir_all(output_dir);
    let _ = std::fs::create_dir_all(output_dir);


    let input_system_file = format!("{}/Input system",output_dir);
    let _ = std::fs::create_dir_all(input_system_file.clone());
    
    let product_file = format!("{}/Product",output_dir);
    let _ = std::fs::create_dir_all(product_file.clone());
    
    match communicating_automata_parse_file(file_name) {

        Ok(caut_system) => {

            println!(
                "\n=======================================================================\n\
                   The system of Concurrent Automata file {} has been successfully parsed \n\
                    =======================================================================\n",file_name
            );
            let file_name_without_extension = if let Some(name) = PathBuf::from(file_name).file_stem().unwrap().to_str(){
                name.to_string()
            }
            else{
                file_name.to_string()
            };
            /*****/
            caut_system.draw(&file_name_without_extension, &input_system_file.to_string());
            
            //Computation of the synchronous product
            match SyncProduct::build_sync_product(&caut_system){
                Ok(prod) => {
                    println!("The synchronous product has been successfully computed");
                    prod.draw(&file_name_without_extension, &product_file.to_string());
                    
                },
                Err(e) => {
                    println!("The synchronous product has not been successfully computed");
                    println!("Error: {}", e);
                },
            }
            
            println!("The output is in the following folder: {}",output_dir);
        }
        Err(e) => {
            println!("Error: {}", e);
        }

    }
}