use std::fs;
use clap::ArgMatches;
use graphviz_dot_builder::traits::DotTranslatable;
use crate::automata::algos::automata_mining::ktssi::KTSSI;
use crate::regex_inf::sample::sample::Sample;

pub fn cli_ktssi(matches: &ArgMatches) {


    let k = matches.value_of("k").unwrap().parse::<usize>().unwrap();

    println!("Automaton leaning with KTSSI with k = {}",k);
    println!("--------------------------------------------------");
    println!("--------------------------------------------------");
    //Sample parsing
    let mut sample = Sample::new();
    let separator = match matches.value_of("separator"){
        Some(s) if s.len()==1 => Some(s.chars().next().unwrap()),
        _ => None
    };

    if let Some(text_file) = matches.value_of("text_input") {
        //parse the text file
        let words = Sample::parse_txt_file(text_file);
        sample = Sample::new_from_set(&words,separator)
    }

    println!("Sample parsing successful");

    if let Some(sep) = separator {
        println!("Automaton inference with the following sample with the separator '{}' between alphabet symbols",sep);
    }
    else{
        println!("Automaton inference with the following sample with no separators between alphabet symbols");
    }
    println!("--------------------------------------------------");
    println!("--------------------------------------------------");
    println!("Sample: {}",sample);
    println!("--------------------------------------------------");
    println!("--------------------------------------------------");
    //Mining
    println!("Starting Inference...");
    println!();

    let automaton = KTSSI::ktssi(k,&sample);

    //println!("Result of KTSSI : {:?}",automaton.to_dot().to_dot_string());
    println!("--------------------------------------------------");
    println!("Creating a dot file...");
    println!();

    let output_folder = "KTSSI_Results";
    let _ = fs::remove_dir_all(output_folder);
    let _ = fs::create_dir(output_folder);

    let dot_file = output_folder.to_string() + "/automaton.dot";
    let _ = fs::write(dot_file, automaton.to_dot().to_dot_string());

    println!("Dot file created. Check the folder {} for the result",output_folder);

}