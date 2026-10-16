use std::alloc::System;
use clap::ArgMatches;
use crate::automata::cfsm::system_cfsm::system_cfsm::SystemCFSM;

pub fn cli_cfsm_composition_benchmark(matches: &ArgMatches){
    let folder_name = matches.value_of("folder").unwrap();

    SystemCFSM::run_benchmarking_cfsm_composition(folder_name);
}