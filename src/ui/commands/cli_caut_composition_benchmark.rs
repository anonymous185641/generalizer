use clap::ArgMatches;
use crate::automata::communicating_automata::composition::benchmark::caut_comp_benchmark::CautCompBenchmark;

pub fn cli_caut_composition_benchmark(matches: &ArgMatches){

    let dataset_folder = matches.value_of("dataset_folder").unwrap();

    let target_folder = "TACAS_2027_BENCHMARK_output";
    let _ = std::fs::remove_dir_all(target_folder);
    let _ = std::fs::create_dir_all(target_folder);

    //We always activate the variants for now
    let comp_timeout_secs = Some(60);
    let comp_with_variants = true;
    let drawings = true;

    let benchmark_runner = CautCompBenchmark::new(dataset_folder.to_string(),target_folder.to_string(),comp_with_variants,drawings,comp_timeout_secs);


    println!(
        "\n=======================================================================\n\
                Beginning the Benchmarking of use cases in the folder: {} \n\
          =======================================================================\n",dataset_folder
    );


    match benchmark_runner.run(){
        Ok(_) => {
            println!(
                "\n=======================================================================\n\
                        The benchmark has been successfully executed. \n\
                  =======================================================================\n"
            );

        },
        Err(e) => {
            println!(
                "\n=======================================================================\n\
                        An error occurred during the execution of the benchmark: {} \n\
                  =======================================================================\n",e
            );
        }
    }
    
}