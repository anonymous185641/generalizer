use std::fs;
use std::path::Path;
use clap::ArgMatches;
use crate::interactions::io::input::hif::interface::parse_hif_file;
use crate::interactions::io::input::hsf::interface::parse_hsf_file;
use crate::interactions::io::output::quick_drawing::draw_model_without_hif;

pub fn cli_draw_interaction_model(matches: &ArgMatches) {
    let hsf_file = matches.value_of("hsf").unwrap();
    let int_file = matches.value_of("hif").unwrap();


    let gen_ctx = match parse_hsf_file(hsf_file){
        Ok(sig) => sig,
        Err(e) =>{
            println!("Error parsing hsf file: {}",e);
            panic!("Could not parse HSF file");
        }
    };

    let i = match parse_hif_file(&gen_ctx,int_file){
        Ok(int) => int,
        Err(e)=>{
            println!("Error parsing hsf file: {}",e);
            panic!("Could not parse HIF file");
        }
    };

    let name = Path::new(int_file)
        .file_stem()
        .unwrap()
        .to_string_lossy();



    let draw_dir = "draw_interaction_output";
    let _ = fs::remove_dir_all(draw_dir).ok();
    let _ = fs::create_dir_all(draw_dir).ok();
    println!("Drawing the interaction model {}",name);
    println!("Output directory: {}\n",draw_dir);

    draw_model_without_hif(&gen_ctx,&name,draw_dir,&i);
    
}