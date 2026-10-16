use std::fs;
use std::path::Path;
use std::time::Instant;
use crate::interaction_mining::experiments::experiment1::exp1_output::{Exp1CompVerdict, Exp1Line, Exp1Output};
use crate::interactions::composition::error::CompositionError;
use crate::interactions::io::output::quick_drawing::draw_model;
use crate::interactions::syntax::action::Action;
use crate::interactions::syntax::interaction::Interaction;
use crate::regex_inf::algos::con_miner::ConMiner;
use crate::regex_inf::algos::crx::CRX;
use crate::regex_inf::algos::gen_ichare::GenIchare;
use crate::regex_inf::algos::soa2chare::Soa2Chare;
use crate::regex_inf::algos::soa2soire::Soa2Soire;
use crate::regex_inf::algos::soa2sore::Soa2Sore;
use crate::regex_inf::sample::sample::Sample;
use crate::terms::function::Axioms;

pub struct Experiment1 {
    pub folder: String,
    pub timout_secs: Option<f64>,
    pub millis: bool,
    pub interleaving_only: bool,
}

impl Experiment1 {
    pub fn new(folder: &str,timout_secs:Option<f64>,millis:bool,interleaving_only:bool) -> Experiment1 {
        Experiment1 {folder: String::from(folder),timout_secs, millis,interleaving_only}
    }

    pub fn init(&self){
        todo!()
    }
    pub fn run(&self){

        let folder_path = Path::new(&self.folder);

        //Output_folder
        let output_folder = "Experiment1_Output";
        fs::remove_dir_all(output_folder).ok();
        fs::create_dir_all(folder_path).unwrap();

        //Output
        let mut exp_output = Exp1Output::new();

        for entry in fs::read_dir(folder_path).unwrap() {
            let sub_folder_path = entry.unwrap().path();
            let sub_folder_path_str = sub_folder_path.to_str().unwrap();
            let sub_folder_name = sub_folder_path.file_name().unwrap().to_str().unwrap();

            let interaction_output_folder_name = format!("{}/{}", output_folder, sub_folder_name);
            fs::create_dir_all(&interaction_output_folder_name).unwrap();

            match Action::gen_multitraces(sub_folder_path_str,output_folder,None){
                Ok((mt,int_info))=>{
                    let samples = Sample::multitraces_to_samples(&mt);

                    let mut locals_from_chare = Vec::new();
                    let mut locals_from_sore = Vec::new();
                    let mut locals_from_crx = Vec::new();
                    let mut locals_from_ichare = Vec::new();
                    let mut locals_from_con_miner = Vec::new();
                    let mut locals_from_soire = Vec::new();

                    let mut regex_times : Vec<f64> = vec![0.0,0.0,0.0,0.0,0.0,0.0];

                    ////////////////////////////////////////////////////

                    for (lf_id,sample) in samples.iter() {
                        println!("{:?}",sample);
                        //Soa2Chare
                        let mut now = Instant::now();
                        let chare = Soa2Chare::i_chare(&sample);
                        let mut elapsed = now.elapsed();
                        if *lf_id>0{
                            regex_times[0] = (regex_times[0] + elapsed.as_secs_f64())/2.0;
                        }
                        else{
                            regex_times[0] = elapsed.as_secs_f64()
                        }
                        //Soa2Sore
                        now = Instant::now();
                        let sore = Soa2Sore::i_sore(&sample);
                        let mut elapsed = now.elapsed();
                        if *lf_id>0{
                            regex_times[1] = (regex_times[1] + elapsed.as_secs_f64())/2.0;
                        }
                        else{
                            regex_times[1] = elapsed.as_secs_f64()
                        }

                        //CRX
                        now = Instant::now();
                        let crx = CRX::crx(&sample);
                        let mut elapsed = now.elapsed();
                        if *lf_id>0{
                            regex_times[2] = (regex_times[2] + elapsed.as_secs_f64())/2.0;
                        }
                        else{
                            regex_times[2] = elapsed.as_secs_f64()
                        }


                        //GenICHARE
                        now = Instant::now();
                        let ichare = GenIchare::i_ichare(&sample);
                        let mut elapsed = now.elapsed();
                        if *lf_id>0{
                            regex_times[3] = (regex_times[3] + elapsed.as_secs_f64())/2.0;
                        }
                        else{
                            regex_times[3] = elapsed.as_secs_f64()
                        }

                        //ConMiner
                        now = Instant::now();
                        let con_miner = ConMiner::con_miner(&sample);
                        let mut elapsed = now.elapsed();
                        if *lf_id>0{
                            regex_times[4] = (regex_times[4] + elapsed.as_secs_f64())/2.0;
                        }
                        else{
                            regex_times[4] = elapsed.as_secs_f64()
                        }


                        //Soa2Soire
                        now = Instant::now();
                        let soire = Soa2Soire::i_soire(&sample);
                        elapsed = now.elapsed();
                        if *lf_id>0{
                            regex_times[5] = (regex_times[5] + elapsed.as_secs_f64())/2.0;
                        }
                        else{
                            regex_times[5] = elapsed.as_secs_f64()
                        }

                        println!("con_miner: {:?}",con_miner);//.to_text(Some('.'))
                        println!("crx_chare: {:?}",crx);
                        println!("chare: {:?}",chare);
                        println!("ichare: {:?}",ichare);
                        println!("sore: {:?}",sore);
                        println!("soire: {:?}",soire);

                        let int_chare = Interaction::regex_to_interaction_formatted(&chare).unwrap();
                        let int_sore = Interaction::regex_to_interaction_formatted(&sore).unwrap();
                        let int_crx = Interaction::regex_to_interaction_formatted(&crx).unwrap();
                        let int_ichare = Interaction::regex_to_interaction_formatted(&ichare).unwrap();
                        let int_con_miner = Interaction::regex_to_interaction_formatted(&con_miner).unwrap();
                        let int_soire = Interaction::regex_to_interaction_formatted(&soire).unwrap();


                        locals_from_chare.push((chare,int_chare));
                        locals_from_sore.push((sore,int_sore));
                        locals_from_crx.push((crx,int_crx));
                        locals_from_ichare.push((ichare,int_ichare));
                        locals_from_con_miner.push((con_miner,int_con_miner));
                        locals_from_soire.push((soire,int_soire));

                    }


                    //Drawing of locals
                    let local_dir = format!("{}/Locals", &interaction_output_folder_name);
                    let local_from_chare_dir = format!("{}/Local_chare", local_dir);
                    let local_from_sore_dir = format!("{}/Local_Sore", local_dir);
                    let local_from_crx_dir = format!("{}/Local_crx", local_dir);
                    let local_from_ichare_dir = format!("{}/Local_Ichare", local_dir);
                    let local_from_con_miner_dir = format!("{}/Local_con_miner", local_dir);
                    let local_from_soire_dir = format!("{}/Local_Soire", local_dir);

                    let _ = fs::create_dir_all(&local_dir).unwrap();
                    let _ = fs::create_dir_all(&local_from_chare_dir).unwrap();
                    let _ = fs::create_dir_all(&local_from_sore_dir).unwrap();
                    let _ = fs::create_dir_all(&local_from_crx_dir).unwrap();
                    let _ = fs::create_dir_all(&local_from_ichare_dir).unwrap();
                    let _ = fs::create_dir_all(&local_from_con_miner_dir).unwrap();
                    let _ = fs::create_dir_all(&local_from_soire_dir).unwrap();


                    for ct in 0..locals_from_chare.len(){
                        draw_model(&int_info.gen_ctx,&format!("i{}",ct+1),
                                   &local_from_chare_dir,
                                   &locals_from_chare[ct].1);
                        let _ = locals_from_chare[ct].0.to_file(&local_from_chare_dir,&format!("reg{}",ct+1));

                        draw_model(&int_info.gen_ctx,&format!("i{}",ct+1),
                                   &local_from_sore_dir,
                                   &locals_from_sore[ct].1);
                        let _ = locals_from_sore[ct].0.to_file(&local_from_sore_dir,&format!("reg{}",ct+1));

                        draw_model(&int_info.gen_ctx,&format!("i{}",ct+1),
                                   &local_from_crx_dir,
                                   &locals_from_crx[ct].1);
                        let _ = locals_from_crx[ct].0.to_file(&local_from_crx_dir,&format!("reg{}",ct+1));

                        draw_model(&int_info.gen_ctx,&format!("i{}",ct+1),
                                   &local_from_ichare_dir,
                                   &locals_from_ichare[ct].1);
                        let _ = locals_from_ichare[ct].0.to_file(&local_from_ichare_dir,&format!("reg{}",ct+1));

                        draw_model(&int_info.gen_ctx,&format!("i{}",ct+1),
                                   &local_from_con_miner_dir,
                                   &locals_from_con_miner[ct].1);
                        let _ = locals_from_con_miner[ct].0.to_file(&local_from_con_miner_dir,&format!("reg{}",ct+1));

                        draw_model(&int_info.gen_ctx,&format!("i{}",ct+1),
                                   &local_from_soire_dir,
                                   &locals_from_soire[ct].1);
                        let _ = locals_from_soire[ct].0.to_file(&local_from_soire_dir,&format!("reg{}",ct+1));
                    }


                    //Tagging
                    let chare_tagged_local_ints = Interaction::tagging_pairwise(&locals_from_chare[0].1, &locals_from_chare[1].1);
                    let sore_tagged_local_ints = Interaction::tagging_pairwise(&locals_from_sore[0].1, &locals_from_sore[1].1);
                    let crx_tagged_local_ints = Interaction::tagging_pairwise(&locals_from_crx[0].1, &locals_from_crx[1].1);
                    let ichare_tagged_local_ints = Interaction::tagging_pairwise(&locals_from_ichare[0].1, &locals_from_ichare[1].1);
                    let con_miner_tagged_local_ints = Interaction::tagging_pairwise(&locals_from_con_miner[0].1, &locals_from_con_miner[1].1);
                    let soire_tagged_local_ints = Interaction::tagging_pairwise(&locals_from_soire[0].1, &locals_from_soire[1].1);

                    //Drawing of tagged locals
                    let tagged_local_dir = format!("{}/Tagged Locals", &interaction_output_folder_name);
                    let tagged_local_from_chare_dir = format!("{}/Local_chare", tagged_local_dir);
                    let tagged_local_from_sore_dir = format!("{}/Local_Sore", tagged_local_dir);
                    let tagged_local_from_crx_dir = format!("{}/Local_Crx", tagged_local_dir);
                    let tagged_local_from_ichare_dir = format!("{}/Local_Ichare", tagged_local_dir);
                    let tagged_local_from_con_miner_dir = format!("{}/Local_con_miner", tagged_local_dir);
                    let tagged_local_from_soire_dir = format!("{}/Local_Soire", tagged_local_dir);


                    let _ = fs::create_dir_all(&tagged_local_dir).unwrap();
                    let _ = fs::create_dir_all(&tagged_local_from_chare_dir).unwrap();
                    let _ = fs::create_dir_all(&tagged_local_from_sore_dir).unwrap();
                    let _ = fs::create_dir_all(&tagged_local_from_crx_dir).unwrap();
                    let _ = fs::create_dir_all(&tagged_local_from_ichare_dir).unwrap();
                    let _ = fs::create_dir_all(&tagged_local_from_con_miner_dir).unwrap();
                    let _ = fs::create_dir_all(&tagged_local_from_soire_dir).unwrap();


                    for ct in 0..locals_from_chare.len(){ //basically 0..2
                        draw_model(&int_info.gen_ctx,&format!("i{}",ct+1),
                                   &tagged_local_from_chare_dir,
                                   &chare_tagged_local_ints[ct]);

                        draw_model(&int_info.gen_ctx,&format!("i{}",ct+1),
                                   &tagged_local_from_sore_dir,
                                   &sore_tagged_local_ints[ct]);

                        draw_model(&int_info.gen_ctx,&format!("i{}",ct+1),
                                   &tagged_local_from_crx_dir,
                                   &crx_tagged_local_ints[ct]);

                        draw_model(&int_info.gen_ctx,&format!("i{}",ct+1),
                                   &tagged_local_from_ichare_dir,
                                   &ichare_tagged_local_ints[ct]);

                        draw_model(&int_info.gen_ctx,&format!("i{}",ct+1),
                                   &tagged_local_from_con_miner_dir,
                                   &con_miner_tagged_local_ints[ct]);

                        draw_model(&int_info.gen_ctx,&format!("i{}",ct+1),
                                   &tagged_local_from_soire_dir,
                                   &soire_tagged_local_ints[ct]);
                    }

                    //Composition
                    let mut comp_time:Vec<Exp1CompVerdict> = Vec::new();

                    let comp_dir = format!("{}/Composition result", &interaction_output_folder_name);
                    let comp_chare_dir = format!("{}/Composition_chare", comp_dir);
                    let comp_sore_dir = format!("{}/Composition_Sore", comp_dir);
                    let comp_crx_dir = format!("{}/Composition_Crx", comp_dir);
                    let comp_ichare_dir = format!("{}/Composition_Ichare", comp_dir);
                    let comp_con_miner_dir = format!("{}/Composition_ConMiner", comp_dir);
                    let comp_soire_dir = format!("{}/Composition_Soire", comp_dir);


                    let _ = fs::create_dir_all(&comp_dir).unwrap();
                    let _ = fs::create_dir_all(&comp_chare_dir).unwrap();
                    let _ = fs::create_dir_all(&comp_sore_dir).unwrap();
                    let _ = fs::create_dir_all(&comp_crx_dir).unwrap();
                    let _ = fs::create_dir_all(&comp_ichare_dir).unwrap();
                    let _ = fs::create_dir_all(&comp_con_miner_dir).unwrap();
                    let _ = fs::create_dir_all(&comp_soire_dir).unwrap();



                    let global_interaction_norm = int_info.interaction.iat_canonize(&int_info.gen_ctx).clean_gates();

                    //CHARE composition
                    let mut now = Instant::now();
                    match Interaction::compose(&chare_tagged_local_ints[0],
                                               &chare_tagged_local_ints[1],
                                               /*false,
                                               false,
                                               true,
                                               self.timout_secs,*/
                                               &Axioms::acu(),false,None){
                        Ok(result) => {
                            let elapsed = now.elapsed();

                            println!("CHARE composition successful");
                            draw_model(&int_info.gen_ctx,"comp_result",
                                       &comp_chare_dir,
                                       &result);

                            //Multitrace checking and verdict
                            let result_int_dir = format!("{}/comp_result.hif",comp_chare_dir);
                            if let Ok(rate) = Self::multitrace_check(result_int_dir.as_str(),&int_info){
                                if result.iat_canonize(&int_info.gen_ctx) == global_interaction_norm {
                                    comp_time.push(Exp1CompVerdict::Success(true,elapsed.as_secs_f64(),rate));
                                }
                                else{
                                    comp_time.push(Exp1CompVerdict::Success(false,elapsed.as_secs_f64(),rate));
                                }
                            }
                            else{
                                panic!("Could not check multitrace membership on result interaction");
                            }
                        },
                        Err(CompositionError::TimedOut) => {
                            comp_time.push(Exp1CompVerdict::Timout);
                            println!("Timout");
                        },
                        Err(e)=>{
                            comp_time.push(Exp1CompVerdict::Failure);
                            println!("{}", e);
                        }
                    }


                    //SORE composition
                    now = Instant::now();
                    match Interaction::compose(&sore_tagged_local_ints[0],
                                               &sore_tagged_local_ints[1],
                                               /*false,
                                               false,
                                               true,
                                               self.timout_secs,*/
                                               &Axioms::acu(),false,None){
                        Ok(result) => {
                            let elapsed = now.elapsed();

                            println!("SORE composition successful");
                            draw_model(&int_info.gen_ctx,"comp_result",
                                       &comp_sore_dir,
                                       &result);

                            //Multitrace checking and verdict
                            let result_int_dir = format!("{}/comp_result.hif",comp_sore_dir);
                            if let Ok(rate) = Self::multitrace_check(result_int_dir.as_str(),&int_info){
                                if result.iat_canonize(&int_info.gen_ctx).clean_gates() == global_interaction_norm {
                                    comp_time.push(Exp1CompVerdict::Success(true,elapsed.as_secs_f64(),rate));
                                }
                                else{
                                    comp_time.push(Exp1CompVerdict::Success(false,elapsed.as_secs_f64(),rate));
                                }
                            }
                            else{
                                panic!("Could not check multitrace membership on result interaction");
                            }
                        },
                        Err(CompositionError::TimedOut) => {
                            comp_time.push(Exp1CompVerdict::Timout);
                            println!("Timout");
                        },
                        Err(e)=>{
                            comp_time.push(Exp1CompVerdict::Failure);
                            println!("{}", e);
                        }
                    }

                    //CRX composition
                    now = Instant::now();
                    match Interaction::compose(&crx_tagged_local_ints[0],
                                               &crx_tagged_local_ints[1],
                                               /*false,
                                               false,
                                               true,
                                               self.timout_secs,*/
                                               &Axioms::acu(),false,None){
                        Ok(result) => {
                            let elapsed = now.elapsed();

                            println!("SORE composition successful");
                            draw_model(&int_info.gen_ctx,"comp_result",
                                       &comp_crx_dir,
                                       &result);

                            //Multitrace checking and verdict
                            let result_int_dir = format!("{}/comp_result.hif",comp_crx_dir);
                            if let Ok(rate) = Self::multitrace_check(result_int_dir.as_str(),&int_info){
                                if result.iat_canonize(&int_info.gen_ctx).clean_gates() == global_interaction_norm {
                                    comp_time.push(Exp1CompVerdict::Success(true,elapsed.as_secs_f64(),rate));
                                }
                                else{
                                    comp_time.push(Exp1CompVerdict::Success(false,elapsed.as_secs_f64(),rate));
                                }
                            }
                            else{
                                panic!("Could not check multitrace membership on result interaction");
                            }
                        },
                        Err(CompositionError::TimedOut) => {
                            comp_time.push(Exp1CompVerdict::Timout);
                            println!("Timout");
                        },
                        Err(e)=>{
                            comp_time.push(Exp1CompVerdict::Failure);
                            println!("{}", e);
                        }
                    }


                    //ICHARE composition
                    now = Instant::now();
                    match Interaction::compose(&ichare_tagged_local_ints[0],
                                               &ichare_tagged_local_ints[1],
                                               /*false,
                                               false,
                                               true,
                                               self.timout_secs,*/
                                               &Axioms::acu(),false,None){
                        Ok(result) => {
                            let elapsed = now.elapsed();

                            println!("ICHARE composition successful");
                            draw_model(&int_info.gen_ctx,"comp_result",
                                       &comp_ichare_dir,
                                       &result);

                            //Multitrace checking and verdict
                            let result_int_dir = format!("{}/comp_result.hif",comp_ichare_dir);
                            if let Ok(rate) = Self::multitrace_check(result_int_dir.as_str(),&int_info){
                                if result.iat_canonize(&int_info.gen_ctx).clean_gates() == global_interaction_norm {
                                    comp_time.push(Exp1CompVerdict::Success(true,elapsed.as_secs_f64(),rate));
                                }
                                else{
                                    comp_time.push(Exp1CompVerdict::Success(false,elapsed.as_secs_f64(),rate));
                                }
                            }
                            else{
                                panic!("Could not check multitrace membership on result interaction");
                            }
                        },
                        Err(CompositionError::TimedOut) => {
                            comp_time.push(Exp1CompVerdict::Timout);
                            println!("Timout");
                        },
                        Err(e)=>{
                            comp_time.push(Exp1CompVerdict::Failure);
                            println!("{}", e);
                        }
                    }

                   //ConMiner composition
                    now = Instant::now();
                    match Interaction::compose(&con_miner_tagged_local_ints[0],
                                               &con_miner_tagged_local_ints[1],
                                               /*false,
                                               false,
                                               true,
                                               self.timout_secs,*/
                                               &Axioms::acu(),false,None){
                        Ok(result) => {
                            let elapsed = now.elapsed();

                            println!("SORE composition successful");
                            draw_model(&int_info.gen_ctx,"comp_result",
                                       &comp_con_miner_dir,
                                       &result);

                            //Multitrace checking and verdict
                            let result_int_dir = format!("{}/comp_result.hif",comp_con_miner_dir);
                            if let Ok(rate) = Self::multitrace_check(result_int_dir.as_str(),&int_info){
                                if result.iat_canonize(&int_info.gen_ctx).clean_gates() == global_interaction_norm {
                                    comp_time.push(Exp1CompVerdict::Success(true,elapsed.as_secs_f64(),rate));
                                }
                                else{
                                    comp_time.push(Exp1CompVerdict::Success(false,elapsed.as_secs_f64(),rate));
                                }
                            }
                            else{
                                panic!("Could not check multitrace membership on result interaction");
                            }
                        },
                        Err(CompositionError::TimedOut) => {
                            comp_time.push(Exp1CompVerdict::Timout);
                            println!("Timout");
                        },
                        Err(e)=>{
                            comp_time.push(Exp1CompVerdict::Failure);
                            println!("{}", e);
                        }
                    }

                    //SOIRE composition
                    now = Instant::now();
                    match Interaction::compose(&soire_tagged_local_ints[0],
                                               &soire_tagged_local_ints[1],
                                               /*false,
                                               false,
                                               true,
                                               self.timout_secs,*/
                                               &Axioms::acu(),false,None){
                        Ok(result) => {
                            let elapsed = now.elapsed();

                            println!("SOIRE composition successful");
                            draw_model(&int_info.gen_ctx,"comp_result",
                                       &comp_soire_dir,
                                       &result);

                            //Multitrace checking and verdict
                            let result_int_dir = format!("{}/comp_result.hif",comp_soire_dir);
                            if let Ok(rate) = Self::multitrace_check(result_int_dir.as_str(),&int_info){
                                if result.iat_canonize(&int_info.gen_ctx).clean_gates() == global_interaction_norm {
                                    comp_time.push(Exp1CompVerdict::Success(true,elapsed.as_secs_f64(),rate));
                                }
                                else{
                                    comp_time.push(Exp1CompVerdict::Success(false,elapsed.as_secs_f64(),rate));
                                }
                            }
                            else{
                                panic!("Could not check multitrace membership on result interaction");
                            }
                        },
                        Err(CompositionError::TimedOut) => {
                            comp_time.push(Exp1CompVerdict::Timout);
                            println!("Timout");
                        },
                        Err(e)=>{
                            comp_time.push(Exp1CompVerdict::Failure);
                            println!("{}", e);
                        }
                    }

                    //Output
                    let line = Exp1Line::new(&sub_folder_name.to_string(),
                                             int_info.interaction.size(),
                                             int_info.number_of_multitraces,
                                             &regex_times,
                                             &comp_time
                                             ).normalize_durations(self.millis);
                    exp_output.add_line(&line);
                },
                Err(e)=>{
                    panic!("{}", e);
                }
            }
        }

        exp_output.sort();
        if self.interleaving_only{
            exp_output.to_csv_interleaving_only(output_folder,self.millis);
        }
        else{
            exp_output.to_csv(output_folder,self.millis);
        }


    }
}
