use std::fs;
use std::path::Path;
use std::time::{Duration, Instant};
use crate::interaction_mining::experiments::experiment3::exp3_output::{Exp3Line, Exp3Output};
use crate::regex_inf::algos::regex_algos::RegexAlgos;
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

pub struct Experiment3 {
    pub folder: String,
    pub timeout_pairwise_comp_secs: Option<f64>,
    pub global_timeout_secs: Option<f64>,
    pub millis: bool,
    pub max_trace_number: Option<usize>,
    pub trace_ratio: f64,

}

impl Experiment3 {
    pub fn new(folder: &str,timout_pairwise_comp_secs:Option<f64>,global_timout_secs:Option<f64>,millis:bool,max_trace_number:Option<usize>,trace_ratio:f64,) -> Experiment3 {
        Experiment3 {folder: String::from(folder), timeout_pairwise_comp_secs: timout_pairwise_comp_secs, global_timeout_secs: global_timout_secs, millis, max_trace_number,trace_ratio}
    }

    pub fn run(&self){

        let folder_path = Path::new(&self.folder);

        //Output_folder
        let output_folder = "Experiment3_Output";
        fs::remove_dir_all(output_folder).ok();
        fs::create_dir_all(folder_path).unwrap();

        //Output
        let mut exp_output = Exp3Output::new();

        for entry in fs::read_dir(folder_path).unwrap() {
            let sub_folder_path = entry.unwrap().path();
            let sub_folder_path_str = sub_folder_path.to_str().unwrap();
            let sub_folder_name = sub_folder_path.file_name().unwrap().to_str().unwrap();

            println!("----------------------------------");
            println!("Processing {}",sub_folder_name);
            println!("----------------------------------");

            let interaction_output_folder_name = format!("{}/{}", output_folder, sub_folder_name);
            fs::create_dir_all(&interaction_output_folder_name).unwrap();

            match Action::gen_multitraces(sub_folder_path_str,output_folder,None){
                Ok((mt,int_info))=>{

                    //let samples = Sample::multitraces_to_samples(&mt);

                    //Samples with proportions
                    let samples = Sample::multitraces_to_samples_with_ratio(&mt,self.trace_ratio);

                    //Duration start
                    let proportion_duration_start = Instant::now();



                    let mut int_array: Vec<Vec<(Interaction,Vec<RegexAlgos>)>> = Vec::new();
                    ////
                    let mut locals_from_chare = Vec::new();
                    let mut locals_from_sore = Vec::new();
                    let mut locals_from_crx = Vec::new();
                    let mut locals_from_ichare = Vec::new();
                    let mut locals_from_con_miner = Vec::new();
                    let mut locals_from_soire = Vec::new();

                    ////////////////////////////////////////////////////

                    for (lf_id,sample) in samples.iter() {
                        //println!("{:?}",sample);
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

                        println!("Regular expression inference successfull");
                     
                        let int_chare = Interaction::regex_to_interaction_formatted(&chare).unwrap();
                        let int_sore = Interaction::regex_to_interaction_formatted(&sore).unwrap();
                        let int_crx = Interaction::regex_to_interaction_formatted(&crx).unwrap();
                        let int_ichare = Interaction::regex_to_interaction_formatted(&ichare).unwrap();
                        let int_con_miner = Interaction::regex_to_interaction_formatted(&con_miner).unwrap();
                        let int_soire = Interaction::regex_to_interaction_formatted(&soire).unwrap();

                        int_array.push(vec![(int_chare.clone(),vec![RegexAlgos::Soa2Chare]),
                                            (int_sore.clone(),vec![RegexAlgos::Soa2Sore]),
                                            (int_crx.clone(),vec![RegexAlgos::CRX]),
                                            (int_ichare.clone(),vec![RegexAlgos::GenICHARE]),
                                            (int_con_miner.clone(),vec![RegexAlgos::ConMiner]),
                                            (int_soire.clone(),vec![RegexAlgos::Soa2Soire])]);

                        locals_from_chare.push((chare,int_chare,sample));
                        locals_from_sore.push((sore,int_sore,sample));
                        locals_from_crx.push((crx,int_crx,sample));
                        locals_from_ichare.push((ichare,int_ichare,sample));
                        locals_from_con_miner.push((con_miner,int_con_miner,sample));
                        locals_from_soire.push((soire,int_soire,sample));

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
                        let _ = locals_from_chare[ct].1.write_metrics_all(&local_from_chare_dir,&format!("i{}_metrics",ct+1).as_str());


                        draw_model(&int_info.gen_ctx,&format!("i{}",ct+1),
                                   &local_from_sore_dir,
                                   &locals_from_sore[ct].1);
                        let _ = locals_from_sore[ct].0.to_file(&local_from_sore_dir,&format!("reg{}",ct+1));
                        let _ = locals_from_sore[ct].1.write_metrics_all(&local_from_sore_dir,&format!("i{}_metrics",ct+1).as_str());

                        draw_model(&int_info.gen_ctx,&format!("i{}",ct+1),
                                   &local_from_crx_dir,
                                   &locals_from_crx[ct].1);
                        let _ = locals_from_crx[ct].0.to_file(&local_from_crx_dir,&format!("reg{}",ct+1));
                        let _ = locals_from_crx[ct].1.write_metrics_all(&local_from_crx_dir,&format!("i{}_metrics",ct+1).as_str());

                        draw_model(&int_info.gen_ctx,&format!("i{}",ct+1),
                                   &local_from_ichare_dir,
                                   &locals_from_ichare[ct].1);
                        let _ = locals_from_ichare[ct].0.to_file(&local_from_ichare_dir,&format!("reg{}",ct+1));
                        let _ = locals_from_ichare[ct].1.write_metrics_all(&local_from_ichare_dir,&format!("i{}_metrics",ct+1).as_str());

                        draw_model(&int_info.gen_ctx,&format!("i{}",ct+1),
                                   &local_from_con_miner_dir,
                                   &locals_from_con_miner[ct].1);
                        let _ = locals_from_con_miner[ct].0.to_file(&local_from_con_miner_dir,&format!("reg{}",ct+1));
                        let _ = locals_from_con_miner[ct].1.write_metrics_all(&local_from_con_miner_dir,&format!("i{}_metrics",ct+1).as_str());

                        draw_model(&int_info.gen_ctx,&format!("i{}",ct+1),
                                   &local_from_soire_dir,
                                   &locals_from_soire[ct].1);
                        let _ = locals_from_soire[ct].0.to_file(&local_from_soire_dir,&format!("reg{}",ct+1));
                        let _ = locals_from_soire[ct].1.write_metrics_all(&local_from_soire_dir,&format!("i{}_metrics",ct+1).as_str());

                    }



                    /*Composition-OLD 2
                    let mut current: Vec<(Interaction,Vec<RegexAlgos>)> = Vec::new();

                    let all_comps_time = Instant::now();
                    if let Some((first, rest)) = int_array.split_first() {
                        current = first.clone();

                        for next in rest.iter() {
                            current = current
                                .iter()
                                .flat_map(|i| {
                                    next.iter().filter_map(|j| {
                                        Interaction::tag_and_compose_pairwise(
                                            &i.0,
                                            &j.0,
                                            &self.timeout_pairwise_comp_secs,
                                        )
                                            .ok()
                                            .map(|r| (r.clean_gates(),i.1.clone().into_iter().chain(j.1.clone()).collect()))
                                    })
                                })
                                .collect();
                        }
                    }
                    */

                    let intermediary_results = format!("{}/intermediary_results",interaction_output_folder_name);
                    let _ = fs::create_dir_all(&intermediary_results).unwrap();


                    let mut current:Vec<(Interaction,Vec<RegexAlgos>)> = int_array[0].clone();
                    for lf_id in 0..int_array.len()-1{
                        let partial_results_folder = format!("{}/Partial_results {:?}",intermediary_results,[0..=lf_id+1]);
                        let _ = fs::create_dir_all(&partial_results_folder).unwrap();

                        let mut ct=0;
                        let mut temp = Vec::new();
                        for i in current.iter(){
                            for j in int_array[lf_id+1].iter(){
                                if let Ok(result) = Interaction::tag_and_compose_pairwise(&i.0,&j.0,&self.timeout_pairwise_comp_secs){
                                    let org: Vec<RegexAlgos> = i.1.clone().into_iter().chain(j.1.clone()).collect();
                                    temp.push((result.clone(),org.clone()));

                                    let res_dir = format!("{}/Result_{}_{}",partial_results_folder,ct+1,lf_id+1);
                                    fs::create_dir_all(&res_dir).unwrap();
                                    draw_model(&int_info.gen_ctx,&format!("Result {}",ct+1),&res_dir,&result);
                                    fs::write(format!("{}/result_origin_{}.txt",res_dir,ct+1),format!("{:?}",org)).unwrap();
                                    ct+=1;

                                }
                            }
                        }
                        current = temp;
                    }

                    //Recording the number of results for each proportion
                    let total_nb_results = current.len();


                    //Normalization
                    /*
                   for ct in 0..current.len(){
                       current[ct].0 = current[ct].0.iat_canonize(&int_info.gen_ctx);
                    }
                    current.sort();
                    current.dedup();

                   //Recording the number of results after normalization
                    let nb_results_after_normalization = current.len();
                    */

                    // proportion duration end
                    let duration_end = (proportion_duration_start.elapsed().as_secs_f64()*100.0).trunc()/100.0;

                    //Drawing all results
                    let compositions_folder = format!("{}/Results",interaction_output_folder_name);
                    fs::create_dir_all(&compositions_folder).unwrap();
                    for (ct,int) in current.iter().enumerate(){
                        let result_dir = format!("{}/Result_{}",compositions_folder,ct+1);
                        fs::create_dir_all(&result_dir).unwrap();
                        draw_model(&int_info.gen_ctx,&format!("Result {}",ct+1),&result_dir,&int.0);
                        fs::write(format!("{}/result_origin_{}.txt",result_dir,ct+1),format!("{:?}",int.1)).unwrap();
                        int.0.write_metrics_global(&int_info.interaction ,&result_dir,"metrics.txt").unwrap();
                    }



                    ////
                    /*
                    let number_sected_multitraces = (self.trace_ratio * mt.len() as f64).ceil() as usize;

                    let new_line = Exp3Line::new(&sub_folder_name.to_string(),
                                                          int_info.interaction.size(),
                                                          int_info.gen_ctx.get_lf_num(),
                                                          mt.len(),
                                                          number_sected_multitraces,
                                                          total_nb_results,
                                                          nb_results_after_normalization,
                                                          duration_end);
                    exp_output.add_line(&new_line);
                    
                     */

                },
                Err(e)=>{
                    panic!("{}", e);
                }
            }
        }


        //Printing the output
        //exp_output.sort();
        //exp_output.to_csv(output_folder,self.millis,self.global_timeout_secs);
        //println!(
        //    "Experiment 2 finished. Output saved in {}/results.csv",
        //    output_folder
        //)
    }
}
