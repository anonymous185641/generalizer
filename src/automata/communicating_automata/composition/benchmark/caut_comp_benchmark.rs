use std::fs;
use crate::automata::communicating_automata::communicating_automata_io::parsing_system_caut::comm_aut_interface::communicating_automata_parse_file;
use crate::automata::communicating_automata::composition::benchmark::caut_benchmark_result::BenchmarkResult;
use crate::automata::communicating_automata::system_cautomata::cautomata::SystemCommunicatingAutomata;

pub struct CautCompBenchmark{
    pub dataset_folder : String,
    pub target_folder : String,
    pub with_variants:bool,
    pub drawings:bool,
    pub pairwise_composition_timout_secs: Option<usize>
}


impl CautCompBenchmark{
    pub fn new(dataset_folder : String,target_folder : String,with_variants:bool,drawings:bool,pairwise_composition_timout_secs: Option<usize>)->Self{

        Self{dataset_folder,target_folder,with_variants, drawings,pairwise_composition_timout_secs}
    }
    
    pub fn run(&self)-> Result<(), Box<dyn std::error::Error>>{

        //Creation of the target folder
        let _ = fs::remove_dir_all(&self.target_folder);
        let _ = fs::create_dir_all(&self.target_folder);

        //Parsing of the dataset
        let use_cases = self.parse_use_cases()?;

        //Execution of the benchmark
        let mut results = Vec::new();

        for use_case in use_cases{
           let result = BenchmarkResult::run_use_case(&use_case,&self.target_folder,self.with_variants,self.pairwise_composition_timout_secs,self.drawings)?;
            results.push(result);
        }

        //Creation of the csv file
       BenchmarkResult::create_csv(&results,&self.target_folder);

        Ok(())
    }

    pub fn parse_use_cases(&self)->Result<Vec<SystemCommunicatingAutomata>, Box<dyn std::error::Error>>{
        let mut use_cases = Vec::new();

        for system in fs::read_dir(&self.dataset_folder)?{
            let system_name = system?.path().to_str().unwrap().to_string();

            println!("Parsing {}",system_name);
            let caut = communicating_automata_parse_file(&system_name)?;

            use_cases.push(caut);

        }

        Ok(use_cases)
    }
}


