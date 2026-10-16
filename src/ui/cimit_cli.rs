use clap::{load_yaml, App};
use crate::ui::commands::cli_aut_to_regex::cli_aut_to_regex;
use crate::ui::commands::cli_caut_composition_benchmark::cli_caut_composition_benchmark;
use crate::ui::commands::cli_cfsm_composition::cli_cfsm_composition;
use crate::ui::commands::cli_cfsm_composition_benchmark::cli_cfsm_composition_benchmark;
use crate::ui::commands::cli_cfsm_to_caut::cli_cfsm_to_caut;
use crate::ui::commands::cli_sclgg::cli_sclgg;
use crate::ui::commands::cli_compose::cli_compose;
use crate::ui::commands::cli_caut_composition::cli_communicating_automata_composition;
use crate::ui::commands::cli_draw_automaton::cli_draw_automaton;
use crate::ui::commands::cli_draw_cfsm::cli_draw_cfsm;
use crate::ui::commands::cli_draw_communicating_automata::cli_draw_communicating_automata;
use crate::ui::commands::cli_draw_interaction_model::cli_draw_interaction_model;
use crate::ui::commands::cli_draw_petri_net::cli_draw_petri_net;
use crate::ui::commands::cli_draw_transition_system::cli_draw_transition_system;
use crate::ui::commands::cli_ktssi::cli_ktssi;
use crate::ui::commands::cli_lgg::cli_lgg;
use crate::ui::commands::cli_parse_trace::cli_parse_traces;
use crate::ui::commands::cli_petri_to_regex::cli_petri_net_to_regex;
use crate::ui::commands::cli_regex_inf::cli_regex_inf;
use crate::ui::commands::cli_sync_caut_composition::cli_synchronous_communicating_automata_composition;
use crate::ui::commands::cli_sync_product::cli_sync_product;
use crate::ui::commands::cli_test::cli_test;
use crate::ui::utils::logo::print_logo;

pub fn cimit_cli() -> i32{
    let yaml = load_yaml!("cimit_cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    //let mut ret_print : Vec<String> = vec![];
    //let mut ret_code : u32 = 1;

    print_logo();

    if let Some(matches) = matches.subcommand_matches("lgg") {
        let _got = cli_lgg(matches);

    } else if let Some(matches) = matches.subcommand_matches("sclgg") {
        let _got = cli_sclgg(matches);
    }
    else if let Some(matches) = matches.subcommand_matches("compose") {
        let _got = cli_compose(matches);
    }
    else if let Some(matches) = matches.subcommand_matches("parse_traces") {
        let _got = cli_parse_traces(matches);
    }
    else if let Some(matches) = matches.subcommand_matches("test") {
        let _got = cli_test(matches);

    }
    else if let Some(matches) = matches.subcommand_matches("regex_inf") {
        let _got = cli_regex_inf(matches);

    }
    else if let Some(matches) = matches.subcommand_matches("ktssi") {
        let _got = cli_ktssi(matches);

    }
    else if let Some(matches) = matches.subcommand_matches("cfsm_composition") {
        let _got = cli_cfsm_composition(matches);
    }
    else if let Some(matches) = matches.subcommand_matches("caut_composition") {
        let _got = cli_communicating_automata_composition(matches);
    }
    else if let Some(matches) = matches.subcommand_matches("sync_caut_composition") {
        let _got = cli_synchronous_communicating_automata_composition(matches);
    }
    else if let Some(matches) = matches.subcommand_matches("sync_product") {
        let _got = cli_sync_product(matches);
    }
    else if let Some(matches) = matches.subcommand_matches("caut_composition_benchmark") {
        let _got = cli_caut_composition_benchmark(matches);
    }
    else if let Some(matches) = matches.subcommand_matches("cfsm_to_caut") {
        let _got = cli_cfsm_to_caut(matches);
    }
    else if let Some(matches) = matches.subcommand_matches("petri_net_to_regex") {
        let _got = cli_petri_net_to_regex(matches);
    }
    else if let Some(matches) = matches.subcommand_matches("aut_to_regex") {
        let _got = cli_aut_to_regex(matches);
    }
    else if let Some(matches) = matches.subcommand_matches("draw_petri_net") {
        let _got = cli_draw_petri_net(matches);
    }
    else if let Some(matches) = matches.subcommand_matches("draw_transition_system") {
        let _got = cli_draw_transition_system(matches);
    }
    else if let Some(matches) = matches.subcommand_matches("draw_automaton") {
        let _got = cli_draw_automaton(matches);
    }
    else if let Some(matches) = matches.subcommand_matches("draw_interaction") {
        let _got = cli_draw_interaction_model(matches);
    }
    else if let Some(matches) = matches.subcommand_matches("draw_cfsm") {
        let _got = cli_draw_cfsm(matches);
    }
    else if let Some(matches) = matches.subcommand_matches("draw_caut") {
        let _got = cli_draw_communicating_automata(matches);
    }
    else if let Some(matches) = matches.subcommand_matches("cfsm_composition_benchmark") {
        let _got = cli_cfsm_composition_benchmark(matches);
    }
    else {
        println!("TYPE help or -h to get a summary of the utilities");

    }


    return 0;
}