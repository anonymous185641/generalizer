use std::fs;
use std::time::Instant;
use clap::ArgMatches;
use colored::Colorize;
use crate::interactions::io::input::hif::interface::parse_hif_file;
use crate::interactions::io::input::hsf::interface::parse_hsf_file;
use crate::interactions::io::output::quick_drawing::draw_model;
use crate::interactions::syntax::interaction::Interaction;
use crate::terms::function::Axioms;
use crate::ui::utils::print_axioms::print_axioms;

pub fn cli_compose(matches: &ArgMatches) {

    let hsf_file = matches.value_of("hsf").unwrap();
    let int1_file = matches.value_of("hif1").unwrap();
    let int2_file = matches.value_of("hif2").unwrap();

    let gen_ctx = match parse_hsf_file(hsf_file){
        Ok(sig) => sig,
        Err(e) =>{
            println!("Error parsing hsf file: {}",e);
            panic!("Could not parse HSF file");
        }
    };

    let i = match parse_hif_file(&gen_ctx,int1_file){
        Ok(int) => int,
        Err(e)=>{
            println!("Error parsing hsf file: {}",e);
            panic!("Could not parse HIF file");
        }
    };


    let j = match parse_hif_file(&gen_ctx,int2_file){
        Ok(int) => int,
        Err(e)=>{
            println!("Error parsing hif file: {}",e);
            panic!("Could not parse HIF file");
        }
    };

    let with_variant = matches.is_present("variant");
    let loggers = matches.is_present("loggers");

    //let axioms = vec![Axioms::A, Axioms::U];

    let axioms = if matches.is_present("A") {
        vec![Axioms::A]
    }
    else if matches.is_present("C") {
        vec![Axioms::C]
    }
    else if matches.is_present("U") {
        vec![Axioms::U]
    }
    else if matches.is_present("AC") {
        vec![Axioms::A, Axioms::C]
    }
    else if matches.is_present("AU") {
        vec![Axioms::A, Axioms::U]
    }
    else if matches.is_present("CU") {
        vec![Axioms::C, Axioms::U]
    }
    else if matches.is_present("ACU") {
        vec![Axioms::A,Axioms::C, Axioms::U]
    }
    else if matches.is_present("S") {
        vec![]
    }
    else{ //By default ACU
        vec![Axioms::A, Axioms::C, Axioms::U]
    };


    //Printing of the axioms considered
    //print_axioms(&axioms);
    println!("Composition using variants");


    let comp_dir = "Interaction Composition Output";
    let _ = fs::remove_dir_all(comp_dir).ok();
    let _ = fs::create_dir_all(comp_dir).ok();

    //Step 0: Drawing inputs
    let inputs_dir = format!("{}/inputs",comp_dir);
    let _ = fs::create_dir_all(inputs_dir.clone()).ok();
    draw_model(&gen_ctx,"i",&inputs_dir,&i);
    draw_model(&gen_ctx,"j",&inputs_dir,&j);

    //Step 1: Composition
    let time = Instant::now();

    let loggers_folder = if loggers {
        Some(comp_dir)
    }
    else{
        None
    };

    match Interaction::compose(&i,&j,&axioms,with_variant,loggers_folder) {
        Ok(comp_int) =>{
            let cleaned_result = comp_int.clean_gates();
            let elapsed = time.elapsed().as_secs_f64();

            println!("{}", "Composition successful".to_string().green());
            println!("Duration: {} s", elapsed);

            let result_dir = format!("{}/result",comp_dir);
            let _ = fs::create_dir_all(result_dir.clone()).ok();
            draw_model(&gen_ctx,"result",&result_dir,&cleaned_result);


        },
        Err(e)=>{
            println!("Composition error: {:?}",e);
        }
    }


}
