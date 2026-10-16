use std::fs;
use std::fs::File;
use std::time::{Duration, Instant};
use graphviz_rust::attributes::color_name::gold;
use crate::automata::communicating_automata::composition::benchmark::caut_benchmark_entry::{BenchmarkCompositionEntry, BrzozowskiCompositionEntry, Pn2regexCompositionEntry};
use crate::automata::communicating_automata::composition::caut_composition_error::CommunicatingAutomataCompositionError;
use crate::automata::communicating_automata::composition::communicating_automata_to_regex::automata_to_regex_algos::AutomataToRegexAlgo;
use crate::automata::communicating_automata::system_cautomata::cautomata::SystemCommunicatingAutomata;
use crate::interactions::syntax::interaction::Interaction;
use crate::utils::misc::min_max_vec;

#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    pub use_case: SystemCommunicatingAutomata,
    pub composition_entry: BenchmarkCompositionEntry,
}

impl BenchmarkResult{
    pub fn new(use_case: &SystemCommunicatingAutomata,composition_entry: &BenchmarkCompositionEntry)->Self{
        Self{use_case: use_case.clone(), composition_entry: composition_entry.clone()}
    }

    pub fn run_use_case(system: &SystemCommunicatingAutomata,target_folder:&String,with_variants:bool,pairwise_comp_timout:Option<usize>,draw:bool)->Result<Self, Box<dyn std::error::Error>>{

        let use_case_folder = format!("{}/{}",target_folder,system.system_name);
        let _ = fs::create_dir_all(use_case_folder.clone());


        //General infos
        let number_automatas = system.automata.len();
        let number_messages = system.context.messages.len();
        let range_automata_size = min_max_vec(&system.automata.clone().into_iter()
            .map(|x| x.size()).collect::<Vec<usize>>());
        let with_interleaving = system.system_name.contains("interleaving");


        //Brzozowsky
        let brzozowsky_entry = match system.composition_system_communicating_automata(&AutomataToRegexAlgo::Brzozowski,&system.system_name,&use_case_folder,with_variants,pairwise_comp_timout,draw){
            Ok((int,bz_entry,_))=>{
                bz_entry.unwrap().clone()
            },
            Err(CommunicatingAutomataCompositionError::CompositionOfLocalInteractionsError(loc_sizes))=>{
                println!("Failure of composition of local interactions with Brzozowski's algorithm.");
                BrzozowskiCompositionEntry::new(loc_sizes.unwrap(),None,None)
            },
            Err(e)=>{
                println!("Error: {}",e);
                panic!("Composition with Brzozowski failed.")
            },
        };


        //Pn2regex
        let pn2regex_entry =  match system.composition_system_communicating_automata(&AutomataToRegexAlgo::ViaPetriNets,&system.system_name,&use_case_folder,with_variants,pairwise_comp_timout,draw){
            Ok((int,_,pn2regex_entry_option))=>{
                pn2regex_entry_option.unwrap().clone()
            },
            Err(_)=>{
                //println!("Error: {}",e);
                //panic!("Composition with Petri Nets failed.")
                Pn2regexCompositionEntry::new(None,None,None,None)
            },
        };


        let composition_entry =
            BenchmarkCompositionEntry::new(
                system.system_name.clone(),
                //with_interleaving,
                number_automatas,
                number_messages,
                range_automata_size,
                brzozowsky_entry,
                pn2regex_entry);

        let result = BenchmarkResult::new(system,&composition_entry);

        Ok(result)
    }

    pub fn create_csv(results: &Vec<BenchmarkResult>,target_folder: &str){
        let file_name = format!("{}/results.csv",target_folder);
        let file = if let Some(f) = File::create(&file_name).ok(){
            f
        }
        else{
            panic!("Error creating CSV file");
        };


        let mut wtr = csv::WriterBuilder::new()
            .delimiter(b'&')
            //.quote_style(csv::QuoteStyle::NonNumeric)
            .from_writer(file);

        //Titles
        let _ = wtr.write_record(&[
            "use_case".to_string(),
            "number_automatas".to_string(),
            "number_gates".to_string(),
            "range_automata_size".to_string(),
            "brzo_range_loc_int_size".to_string(),
            "brzo_time".to_string(),
            "brzo_global_int_size".to_string(),
            //"petrify_inf_time_range".to_string(),
            "petrify_petri_inf_size_range".to_string(),
            "pn2regex_size_loc_int".to_string(),
            "pn2regex_time".to_string(),
            "pn2regex_global_int_size\\\\".to_string(),
        ]);



        //Values

        for result in results.iter(){
            let _ = wtr.write_record(&result.composition_entry.create_csv_record());
        }

        let _ = wtr.flush();
        
    }


}

















