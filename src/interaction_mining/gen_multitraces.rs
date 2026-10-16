use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use crate::interaction_mining::paths::{CONF_FILE_PATH, HIBOU_PATH};
use crate::interactions::io::input::hif::interface::parse_hif_file;
use crate::interactions::io::input::hsf::interface::parse_hsf_file;
use crate::interactions::io::input::htf::interface::parse_htf_file;
use crate::interactions::syntax::action::Action;
use crate::interactions::syntax::colocalizations::CoLocalizations;
use crate::interactions::syntax::general_context::GeneralContext;
use crate::interactions::syntax::interaction::Interaction;
use crate::interactions::syntax::traces::multitrace::MultiTrace;

pub struct InteractionInfo{
    pub gen_ctx: GeneralContext,
    pub interaction: Interaction,
    pub hsf_path:String,
    pub hif_path: String,
    pub hibou_path: String,
    pub multitraces_path: String,
    pub number_of_multitraces: usize,
}

impl InteractionInfo{
    pub fn new(gen_ctx: &GeneralContext,int: &Interaction, hsf_path: &str,hif_path: &str, hibou_path: &str,multitrace_path: &str,number_multitraces:usize)->Self{
        InteractionInfo{gen_ctx: gen_ctx.clone(),
                        interaction: int.clone(),
                        hsf_path: hsf_path.to_string(),
                        hif_path: hif_path.to_string(),
                        hibou_path: hibou_path.to_string(),
                        multitraces_path: multitrace_path.to_string(),
                        number_of_multitraces: number_multitraces}
    }
}

impl Action{
    pub fn gen_multitraces(int_files_dir:&str,output_folder:&str,max_trace_number:Option<usize>)->Result<(Vec<(CoLocalizations,MultiTrace)>,InteractionInfo),String>{
        let mut multitraces = Vec::new();

        let folder_path = Path::new(int_files_dir);
        let folder_name = folder_path.file_name().unwrap().to_str().unwrap();
        let mut gen_ctx = GeneralContext::new();
        let mut global_interaction = Interaction::Empty;

        let mut hsf: Option<PathBuf> = None;
        let mut hif: Option<PathBuf> = None;


        for entry in fs::read_dir(folder_path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();


            match path.extension().and_then(|e| e.to_str()) {
                Some("hsf") => hsf = Some(path),
                Some("hif") => hif = Some(path),
                _ => {}
            }
        }


        if let (Some(hsf_path),Some(hif_path)) = (hsf,hif.clone()) {
            let hsf_path = hsf_path.as_path().to_str().unwrap();
            let hif_path = hif_path.as_path().to_str().unwrap();

            //Parsing hsf file
            if let Ok(ctx) = parse_hsf_file(hsf_path){
                gen_ctx = ctx;
            }
            else{
                return Err("Could not parse hsf file".to_string());
            }
            //Parsing hif file
            if let Some(hif_path) = hif {
                let hif_path = hif_path.as_path().to_str().unwrap();

                if let Ok(int) = parse_hif_file(&gen_ctx,hif_path){
                    global_interaction = int;
                }
                else{
                    return Err("Could not parse hif file".to_string());
                }
            }

            //Lauch exploration
            //let hibou_label_path = format!("{}/hibou_label",int_files_dir);
            //let hibou_label_path = "hibou_label";
            let hcf_path = format!("{}/conf.hcf",int_files_dir);

            let mut process = if let Ok(c) = Command::new(HIBOU_PATH)
                .arg("explore")
                .arg(hsf_path)
                .arg(hif_path)
                .arg(hcf_path)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn(){
                c
            }
            else {
                return Err("Fail to execute de explore command".to_string());
            };

            //let output = process.wait_with_output().expect("Failed to read stdout");
            let _ = process.wait().expect("Failed to read stdout");


            //Copying the trace files in the output folder
            let traces_path_str = format!("tracegen_l1");
            let traces_path_str_target = format!("{}/{}/Multitraces",output_folder,folder_name);
            let _ = fs::create_dir_all(&traces_path_str_target);

            let mut child1 = if let Ok(c) = Command::new("cp")
                .arg("-R")
                .arg(&traces_path_str)
                .arg(&traces_path_str_target)
                .spawn(){
                    c
                }
                else{
                        return Err("Fail to execute cp command".to_string());
                };

            let _ = child1.wait();
            //Parsing of traces

            let traces_path = Path::new(&traces_path_str);

            let mut number_of_multitraces =  0;

            for htf_file in fs::read_dir(traces_path).unwrap(){
                number_of_multitraces += 1;
                
                if let Some(max_trace_number) = max_trace_number {
                    if number_of_multitraces > max_trace_number {
                        break;
                    }
                }
                
                let entry = htf_file.unwrap();
                let path = entry.path();

                let htf_path = path.as_path().to_str().unwrap();

                if let Ok(mult) = parse_htf_file(&gen_ctx, &htf_path){
                    multitraces.push(mult);
                }
            }


            //remove the traces folder
            let _ = fs::remove_dir_all(&traces_path_str).ok();

            //Interaction Info
            let int_info = InteractionInfo::new(&gen_ctx,
                                                &global_interaction,
                                                hsf_path,
                                                hif_path,
                                                &HIBOU_PATH,
                                                &format!("{}/tracegen_l1",traces_path_str_target),
                                                number_of_multitraces);

            Ok((multitraces,int_info))
        }
        else{
            return Err("Could not parse hsf/hif files".to_string());
        }
    }

}






