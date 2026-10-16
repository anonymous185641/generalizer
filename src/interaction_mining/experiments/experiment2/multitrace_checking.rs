use std::fs;
use std::process::{Command, Stdio};
use crate::interaction_mining::experiments::experiment1::experiment1::Experiment1;
use crate::interaction_mining::experiments::experiment2::experiment2::Experiment2;
use crate::interaction_mining::gen_multitraces::InteractionInfo;
use crate::interactions::syntax::interaction::Interaction;

impl Experiment2{
    pub fn multitrace_check(result_path: &str,int_info: &InteractionInfo )-> Result<f64,String> {

        let mut nb_accepted_multitraces = 0;

        for htf_file in fs::read_dir(&int_info.multitraces_path).unwrap(){
            let entry = htf_file.unwrap();
            let path = entry.path();

            let htf_path = path.as_path().to_str().unwrap();

            let mut process = if let Ok(c) = Command::new(&int_info.hibou_path)
                .arg("analyze")
                .arg(&int_info.hsf_path)
                .arg(&int_info.hif_path)
                .arg(htf_path)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn(){
                c
            }
            else {
                return Err("Fail to execute de explore command".to_string());
            };

            let output = process.wait_with_output().expect("Failed to read stdout");

            let output_str = String::from_utf8_lossy(&output.stdout);

            // Check if the output contains the specific string
            if output_str.contains("verdict    : 'Pass'") {
                println!("Test passed!");
                nb_accepted_multitraces+=1;

            } else {
                println!("Test did not pass or verdict not found");
            }
        }


        Ok((nb_accepted_multitraces as f64 / int_info.number_of_multitraces as f64)*100.0)

    }
}