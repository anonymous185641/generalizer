use std::fs;
use std::path::Path;
use std::time::Instant;
use crate::interaction_mining::experiments::experiment4::exp4_output::{Exp4Line, Exp4Output};
use crate::interactions::io::output::quick_drawing::draw_model;
use crate::interactions::io::output::to_hfiles::trace::to_htf::write_multi_trace_into_file;
use crate::interactions::syntax::action::Action;
use crate::interactions::syntax::interaction::Interaction;
use crate::regex_inf::algos::con_miner::ConMiner;
use crate::regex_inf::algos::crx::CRX;
use crate::regex_inf::algos::gen_ichare::GenIchare;
use crate::regex_inf::algos::k_tssi_brzozowski::KTssiBrzozowski;
use crate::regex_inf::algos::k_tssi_state_removal::KTssiStateRemoval;
use crate::regex_inf::algos::k_tssi_transitive_closure::KTssiTransitiveClosure;
use crate::regex_inf::algos::regex_algos::RegexAlgos::{K_TSSI_Brzozowski, K_TSSI_StateRemoval};
use crate::regex_inf::algos::soa2chare::Soa2Chare;
use crate::regex_inf::algos::soa2soire::Soa2Soire;
use crate::regex_inf::algos::soa2sore::Soa2Sore;

pub struct Experiment4 {
    pub folder: String,
    pub max_trace_number: Option<usize>,
    pub trace_ratio: f64,
    pub messages_to_filter: Option<Vec<String>>,
    pub lifelines_to_filter: Option<Vec<String>>,
}

impl Experiment4 {
    pub fn new(folder: &str,max_trace_number:Option<usize>,trace_ratio:f64,messages_to_filter: Option<Vec<String>>,lifelines_to_filter: Option<Vec<String>>) -> Experiment4 {
        Experiment4 {folder: String::from(folder), max_trace_number,trace_ratio,messages_to_filter,lifelines_to_filter}
    }

    pub fn run(&self){

        let folder_path = Path::new(&self.folder);

        //Output_folder
        let output_folder = "Experiment4_Output";
        fs::remove_dir_all(output_folder).ok();
        fs::create_dir_all(folder_path).unwrap();

        //Output
        let mut exp_output = Exp4Output::new();

        for entry in fs::read_dir(folder_path).unwrap() {
            let sub_folder_path = entry.unwrap().path();
            let sub_folder_path_str = sub_folder_path.to_str().unwrap();
            let sub_folder_name = sub_folder_path.file_name().unwrap().to_str().unwrap();

            println!("----------------------------------");
            println!("Processing {}",sub_folder_name);
            println!("----------------------------------");

            let interaction_output_folder_name = format!("{}/{}", output_folder, sub_folder_name);
            fs::create_dir_all(&interaction_output_folder_name).unwrap();

            //Mining Duration
            let duration_start_mining = Instant::now();

            match Action::gen_multitraces(sub_folder_path_str,output_folder,None){
                Ok((mt,int_info))=>{

                    //Drawing the global input model
                    let input_model_dir = format!("{}/Input_model", &interaction_output_folder_name);
                    let _ = fs::create_dir_all(&input_model_dir).unwrap();
                    draw_model(&int_info.gen_ctx, "sub_folder_name",&input_model_dir,&int_info.interaction);


                    //Trace generation
                    println!("Trace generation successfull");
                    println!("Generated {} multitraces",mt.len());
                    //let samples = Sample::multitraces_to_samples(&mt);

                    //Samples with proportions
                   // let samples = Sample::multitraces_to_samples_with_ratio(&mt,self.trace_ratio);


                    let mut sync_traces = Vec::new();

                    let traces_synchronization_folder = format!("{}/multi-trace_synchronization",interaction_output_folder_name);
                    let _ = fs::create_dir_all(&traces_synchronization_folder).unwrap();

                    println!("Begining multi-trace synchronization");
                    for (ct,(coloc,mult)) in mt.iter().enumerate(){

                        let trace_folder = format!("{}/multi-trace_{}", &traces_synchronization_folder, ct+1);
                        let _ = fs::create_dir_all(&trace_folder).unwrap();

                        //Writing the multitrace to file
                        let file_path_str = format!("{}/multitrace_{}.htf",trace_folder,ct+1);
                        let file_path = Path::new(&file_path_str);

                        write_multi_trace_into_file(file_path,&int_info.gen_ctx,coloc,mult);


                        if let Ok(sync_trace) = Interaction::multitrace_synchronization(&int_info.gen_ctx,mult,Some(&trace_folder)){
                            //println!("{:?}",sync_trace);
                            let mut filtered_trace = sync_trace.clone();

                            if let Some(messages_to_filter) = self.messages_to_filter.clone(){
                                filtered_trace = Interaction::sync_trace_filter_messages(&sync_trace,&int_info.gen_ctx,&messages_to_filter);
                            }

                            if let Some(lifelines_to_filter) = self.lifelines_to_filter.clone(){
                                filtered_trace = Interaction::sync_trace_filter_lfs(&sync_trace,&int_info.gen_ctx,&lifelines_to_filter);
                            }

                            sync_traces.push(filtered_trace);
                            // sync_traces.push(sync_trace);
                        }
                        else{
                            println!("Error in a multi-trace synchronization");
                            continue;
                        }
                    }
                    println!("Multi-trace synchronization successfull");
                    println!("Generated {} synchronized traces",sync_traces.len());





                    let sample = Interaction::sync_traces_to_sample(&sync_traces);
                    println!("Sample: {:?}",sample);

                    // Regex inference
                    println!("Begining regular expression inference");
                    //Soa2Chare
                    let chare = Soa2Chare::i_chare(&sample);

                    //Soa2Sore
                    let sore = Soa2Sore::i_sore(&sample);


                    //CRX
                    let crx = CRX::crx(&sample);

                    //GenICHARE
                    let ichare = GenIchare::i_ichare(&sample);

                    //ConMiner
                    let con_miner = ConMiner::con_miner(&sample);

                    //Soa2Soire
                    let soire = Soa2Soire::i_soire(&sample);

                    //2-TSSI-Bzozowski
                    let two_tssi_brzozowski = KTssiBrzozowski::k_tssi_brzozowski(2,&sample);

                    //2-TSSI-StateRemoval
                    let two_tssi_state_removal = KTssiStateRemoval::k_tssi_state_removal(2,&sample);

                    //2-TSSI-TransitiveClosure
                    let two_tssi_trans_closure = KTssiTransitiveClosure::k_tssi_transitive_closure(2,&sample);


                    println!("Regular expressions inference successfull");

                    println!("chare: {}",chare.to_text(Some('.')));
                    println!("sore: {}",sore.to_text(Some('.')));
                    println!("crx: {}",crx.to_text(Some('.')));
                    println!("ichare: {}",ichare.to_text(Some('.')));
                    println!("con_miner: {}",con_miner.to_text(Some('.')));
                    println!("soire: {}",soire.to_text(Some('.')));
                    println!("2-TSSI-Bzozowski: {}",two_tssi_brzozowski.to_text(Some('.')));
                    println!("2-TSSI-StateRemoval: {}",two_tssi_state_removal.to_text(Some('.')));
                    println!("2-TSSI-TransitiveClosure: {}",two_tssi_trans_closure.to_text(Some('.')));

                    println!("Beginning to draw results");

                    let int_chare = Interaction::regex_to_interaction_vp(&chare).unwrap();
                    let int_sore = Interaction::regex_to_interaction_vp(&sore).unwrap();
                    let int_crx = Interaction::regex_to_interaction_vp(&crx).unwrap();
                    let int_ichare = Interaction::regex_to_interaction_vp(&ichare).unwrap();
                    let int_con_miner = Interaction::regex_to_interaction_vp(&con_miner).unwrap();
                    let int_soire = Interaction::regex_to_interaction_vp(&soire).unwrap();
                    let int_two_tssi_brzozowski = Interaction::regex_to_interaction_vp(&two_tssi_brzozowski).unwrap();
                    let int_two_tssi_state_removal = Interaction::regex_to_interaction_vp(&two_tssi_state_removal).unwrap();
                    let int_two_tssi_trans_closure = Interaction::regex_to_interaction_vp(&two_tssi_trans_closure).unwrap();

                    let mining_duration = (duration_start_mining.elapsed().as_secs_f64()*1000.0).trunc()/1000.0;

                    //Drawing of locals
                    let result_dir = format!("{}/Results", &interaction_output_folder_name);
                    let int_chare_dir = format!("{}/Soa2Chare", result_dir);
                    let int_sore_dir = format!("{}/Soa2Sore", result_dir);
                    let int_crx_dir = format!("{}/CRX", result_dir);
                    let int_ichare_dir = format!("{}/GenIchare", result_dir);
                    let int_con_miner_dir = format!("{}/ConMiner", result_dir);
                    let int_soire_dir = format!("{}/Soa2Soire", result_dir);
                    let int_two_tssi_brzozowski_dir = format!("{}/2-TSSI-Bzozowski", result_dir);
                    let int_two_tssi_state_removal_dir = format!("{}/2-TSSI-StateRemoval", result_dir);
                    let int_two_tssi_trans_closure_dir = format!("{}/2-TSSI-TransitiveClosure", result_dir);

                    let _ = fs::create_dir_all(&result_dir).unwrap();
                    let _ = fs::create_dir_all(&int_chare_dir).unwrap();
                    let _ = fs::create_dir_all(&int_sore_dir).unwrap();
                    let _ = fs::create_dir_all(&int_crx_dir).unwrap();
                    let _ = fs::create_dir_all(&int_ichare_dir).unwrap();
                    let _ = fs::create_dir_all(&int_con_miner_dir).unwrap();
                    let _ = fs::create_dir_all(&int_soire_dir).unwrap();
                    let _ = fs::create_dir_all(&int_two_tssi_brzozowski_dir).unwrap();
                    let _ = fs::create_dir_all(&int_two_tssi_state_removal_dir).unwrap();
                    let _ = fs::create_dir_all(&int_two_tssi_trans_closure_dir).unwrap();


                    let mut results_size:Vec<usize> = Vec::new();
                    let mut results_lang_size:Vec<usize> = Vec::new();

                    draw_model(&int_info.gen_ctx, "result_from_chare",
                               &int_chare_dir,
                               &int_chare);
                    let _ = chare.to_file(&int_chare_dir, "chare_reg");
                   // int_chare.write_metrics_all(&int_chare_dir,"metrics_chare.txt");
                    results_size.push(int_chare.size());
                    println!("H1");
                    results_lang_size.push(int_chare.semantic_size());
                    println!("H2");

                    draw_model(&int_info.gen_ctx, "result_from_sore",
                               &int_sore_dir,
                               &int_sore);
                    let _ = sore.to_file(&int_sore_dir, "sore_reg");
                   // int_sore.write_metrics_all(&int_sore_dir,"metrics_sore.txt");
                    results_size.push(int_sore.size());
                    results_lang_size.push(int_sore.semantic_size());

                    draw_model(&int_info.gen_ctx, "result_from_crx",
                               &int_crx_dir,
                               &int_crx);
                    let _ = crx.to_file(&int_crx_dir, "crx_reg");
                   // int_crx.write_metrics_all(&int_crx_dir,"metrics_crx.txt");
                    results_size.push(int_crx.size());
                    results_lang_size.push(int_crx.semantic_size());

                    draw_model(&int_info.gen_ctx, "result_from_ichare",
                               &int_ichare_dir,
                               &int_ichare);
                    let _ = ichare.to_file(&int_ichare_dir, "ichare_reg");
                    //int_ichare.write_metrics_all(&int_ichare_dir,"metrics_ichare.txt");
                    results_size.push(int_ichare.size());
                    results_lang_size.push(int_ichare.semantic_size());

                    draw_model(&int_info.gen_ctx, "result_from_con_miner",
                               &int_con_miner_dir,
                               &int_con_miner);
                    let _ = con_miner.to_file(&int_con_miner_dir, "con_miner_reg");
                    //int_con_miner.write_metrics_all(&int_con_miner_dir,"metrics_con_miner.txt");
                    results_size.push(int_con_miner.size());
                    results_lang_size.push(int_con_miner.semantic_size());

                    draw_model(&int_info.gen_ctx, "result_from_soire",
                               &int_soire_dir,
                               &int_soire);
                    let _ = soire.to_file(&int_soire_dir, "soire_reg");
                   // int_soire.write_metrics_all(&int_soire_dir,"metrics_soire.txt");
                    results_size.push(int_soire.size());
                    results_lang_size.push(int_soire.semantic_size());

                    draw_model(&int_info.gen_ctx, "result_from_2-TSSI-Bzozowski",
                                &int_two_tssi_brzozowski_dir,
                                &int_two_tssi_brzozowski);
                    let _ = two_tssi_brzozowski.to_file(&int_two_tssi_brzozowski_dir, "2-TSSI-Bzozowski_reg");
                    //int_two_tssi_brzozowski.write_metrics_all(&int_two_tssi_brzozowski_dir,"metrics_2-TSSI-Bzozowski.txt");
                    results_size.push(int_two_tssi_brzozowski.size());
                    results_lang_size.push(int_two_tssi_brzozowski.semantic_size());

                    draw_model(&int_info.gen_ctx, "result_from_2-TSSI-StateRemoval",
                                &int_two_tssi_state_removal_dir,
                                &int_two_tssi_state_removal);
                    let _ = two_tssi_state_removal.to_file(&int_two_tssi_state_removal_dir, "2-TSSI-StateRemoval_reg");
                    //int_two_tssi_state_removal.write_metrics_all(&int_two_tssi_state_removal_dir,"metrics_2-TSSI-StateRemoval.txt");
                    results_size.push(int_two_tssi_state_removal.size());
                    results_lang_size.push(int_two_tssi_state_removal.semantic_size());

                    draw_model(&int_info.gen_ctx, "result_from_2-TSSI-TransitiveClosure",
                                &int_two_tssi_trans_closure_dir,
                                &int_two_tssi_trans_closure);
                    let _ = two_tssi_trans_closure.to_file(&int_two_tssi_trans_closure_dir, "2-TSSI-TransitiveClosure_reg");
                  //  int_two_tssi_trans_closure.write_metrics_all(&int_two_tssi_trans_closure_dir,"metrics_2-TSSI-TransitiveClosure.txt");
                    results_size.push(int_two_tssi_trans_closure.size());
                    results_lang_size.push(int_two_tssi_trans_closure.semantic_size());



                    println!("Drawing finised");
                    println!("Check the results in: {}",interaction_output_folder_name);
                    println!("----------------------------------");

                    let line = Exp4Line::new(&sub_folder_name.to_string(),
                                                     int_info.interaction.size(),
                                                      int_info.gen_ctx.get_lf_num(),
                                                      int_info.interaction.semantic_size(),
                                                        mt.len(),
                                                      mining_duration,
                                                      results_size,
                                                      results_lang_size);
                },
                Err(e)=>{
                    panic!("{}", e);
                }
            }
        }



        //Printing the output
        exp_output.sort();
        exp_output.to_csv(output_folder);
        println!(
            "Experiment 4 finished. Output saved in {}/results.csv",
           output_folder
        );

    }
}
