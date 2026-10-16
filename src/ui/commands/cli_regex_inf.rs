use std::fs;
use std::time::Instant;
use clap::{ArgEnum, ArgMatches};
use crate::regex_inf::algos::con_miner::ConMiner;
use crate::regex_inf::algos::crx::CRX;
use crate::regex_inf::algos::gen_ichare::GenIchare;
use crate::regex_inf::algos::k_tssi_brzozowski::KTssiBrzozowski;
use crate::regex_inf::algos::k_tssi_state_removal::KTssiStateRemoval;
use crate::regex_inf::algos::k_tssi_transitive_closure::KTssiTransitiveClosure;
use crate::regex_inf::algos::regex_algos::RegexAlgos::K_TSSI_Brzozowski;
use crate::regex_inf::algos::soa2chare::Soa2Chare;
use crate::regex_inf::algos::soa2soire::Soa2Soire;
use crate::regex_inf::algos::soa2sore::Soa2Sore;
use crate::regex_inf::sample::sample::Sample;

pub fn cli_regex_inf(matches: &ArgMatches) {




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
        println!("Regular expression inference with the following sample with the separator '{}' between alphabet symbols",sep);
    }
    else{
        println!("Regular expression inference with the following sample with no separators between alphabet symbols");
    }
    println!("--------------------------------------------------");
    println!("--------------------------------------------------");
    println!("Sample: {}",sample);
    println!("--------------------------------------------------");
    println!("--------------------------------------------------");
    //Mining
    println!("Starting Inference...");
    println!();

    let start = Instant::now();
    let con_miner = ConMiner::con_miner(&sample).simple_simplify();
    let end = start.elapsed();
    println!("Inference time of the ConMiner algorithm: {:?} ms",end.as_secs_f64() * 1000.0);
    println!("Result of ConMiner : {:?}",con_miner.to_text(separator));
    println!("--------------------------------------------------");

    let start = Instant::now();
    let crx_chare = CRX::crx(&sample).simple_simplify();
    let end = start.elapsed();
    println!("Inference time of the CRX algorithm: {:?} ms",end.as_secs_f64() * 1000.0);
    println!("Result of CRX : {:?}",crx_chare.to_text(separator));
    println!("--------------------------------------------------");

    let start = Instant::now();
    let chare = Soa2Chare::i_chare(&sample).simple_simplify();
    let end = start.elapsed();
    println!("Inference time of the Soa2Chare algorithm: {:?} ms",end.as_secs_f64() * 1000.0);
    println!("Result of Soa2Chare : {:?}",chare.to_text(separator));
    println!("--------------------------------------------------");


    let start = Instant::now();
    let ichare = GenIchare::i_ichare(&sample).simple_simplify();
    let end = start.elapsed();
    println!("Inference time of the GenIchare algorithm: {:?} ms",end.as_secs_f64() * 1000.0);
    println!("Result of GenIchare : {:?}",ichare.to_text(separator));
    println!("--------------------------------------------------");


    let start = Instant::now();
    let sore = Soa2Sore::i_sore(&sample).simple_simplify();
    let end = start.elapsed();
    println!("Inference time of the Soa2Sore algorithm: {:?} ms",end.as_secs_f64() * 1000.0);
    println!("Result of Soa2Sore : {:?}",sore.to_text(separator));
    println!("--------------------------------------------------");


    let start = Instant::now();
    let soire = Soa2Soire::i_soire(&sample).simple_simplify();
    let end = start.elapsed();
    println!("Inference time of the Soa2Soire algorithm: {:?} ms",end.as_secs_f64() * 1000.0);
    println!("Result of Soa2Soire : {:?}",soire.to_text(separator));
    println!("--------------------------------------------------");

    /**/
    let start = Instant::now();
    let bzozowski = KTssiBrzozowski::k_tssi_brzozowski(2,&sample).simple_simplify();
    let end = start.elapsed();
    println!("Inference time of the algorithm 2-TSSI + Brzozowski: {:?} ms",end.as_secs_f64() * 1000.0);
    println!("Result of 2-TSSI + Brzozowski : {:?}",bzozowski.to_text(separator));
    println!("--------------------------------------------------");

    let start = Instant::now();
    let state_removal = KTssiStateRemoval::k_tssi_state_removal(2,&sample).simple_simplify();
    let end = start.elapsed();
    println!("Inference time of the algorithm 2-TSSI + State Removal: {:?} ms",end.as_secs_f64() * 1000.0);
    println!("Result of 2-TSSI + State Removal : {:?}",state_removal.to_text(separator));
    println!("--------------------------------------------------");

    let start = Instant::now();
    let trans_closure = KTssiTransitiveClosure::k_tssi_transitive_closure(2,&sample).simple_simplify();
    let end = start.elapsed();
    println!("Inference time of the algorithm 2-TSSI + Transitive closure: {:?} ms",end.as_secs_f64() * 1000.0);
    println!("Result of 2-TSSI + Transitive closure : {:?}",trans_closure.to_text(separator));
    println!("--------------------------------------------------");





    //Output
    println!("--------------------------------------------------");
    println!("--------------------------------------------------");
    println!("Writing results to files...");

    let output_folder = "Regex_Inference_Results";
    let _ = fs::remove_dir_all(output_folder);
    let _ = fs::create_dir(output_folder);

    let conminer_folder = output_folder.to_string() + "/con_miner";
    let _ = fs::remove_dir_all(&conminer_folder);
    let _ = fs::create_dir(&conminer_folder);
    let conminer_file = conminer_folder.to_string() + "/con_miner.txt";
    let _ = fs::write(conminer_file, con_miner.to_text(separator));


    let crx_chare_folder = output_folder.to_string() + "/crx";
    let _ = fs::create_dir(&crx_chare_folder);
    let crx_chare_file = crx_chare_folder.to_string() + "/crx.txt";
    let _ = fs::write(crx_chare_file, crx_chare.to_text(separator));


    let chare_folder = output_folder.to_string() + "/soa2chare";
    let _ = fs::create_dir(&chare_folder);
    let chare_file = chare_folder.to_string() + "/soa2chare.txt";
    let _ = fs::write(chare_file, chare.to_text(separator));

    let ichare_folder = output_folder.to_string() + "/gen_ichare";
    let _ = fs::create_dir(&ichare_folder);
    let ichare_file = ichare_folder.to_string() + "/gen_ichare.txt";
    let _ = fs::write(ichare_file, ichare.to_text(separator));

    let sore_folder = output_folder.to_string() + "/soa2sore";
    let _ = fs::create_dir(&sore_folder);
    let sore_file = sore_folder.to_string() + "/soa2sore.txt";
    let _ = fs::write(sore_file, sore.to_text(separator));

    let soire_folder = output_folder.to_string() + "/soa2soire";
    let _ = fs::create_dir(&soire_folder);
    let soire_file = soire_folder.to_string() + "/soa2soire.txt";
    let _ = fs::write(soire_file, soire.to_text(separator));

    /* */
    let bzozowski_folder = output_folder.to_string() + "/2-TSSI_Brzozowski";
    let _ = fs::create_dir(&bzozowski_folder);
    let bzozowski_file = bzozowski_folder.to_string() + "/2-TSSI_Brzozowski.txt";
    let _ = fs::write(bzozowski_file, bzozowski.to_text(separator));

    let state_removal_folder = output_folder.to_string() + "/2-TSSI_State_Removal";
    let _ = fs::create_dir(&state_removal_folder);
    let state_removal_file = state_removal_folder.to_string() + "/2-TSSI_State_Removal.txt";
    let _ = fs::write(state_removal_file, state_removal.to_text(separator));

    let trans_closure_folder = output_folder.to_string() + "/2-TSSI_Transitive_Closure";
    let _ = fs::create_dir(&trans_closure_folder);
    let trans_closure_file = trans_closure_folder.to_string() + "/2-TSSI_Transitive_Closure.txt";
    let _ = fs::write(trans_closure_file, trans_closure.to_text(separator));
    


    println!("Results successfully written into files");
    println!("Check the folder {} for the results",output_folder);

}




