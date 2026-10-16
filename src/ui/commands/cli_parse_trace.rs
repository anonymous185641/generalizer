use clap::ArgMatches;
use crate::interactions::io::input::hsf::interface::parse_hsf_file;
use crate::interactions::io::input::htf::interface::parse_htf_file;

pub fn cli_parse_traces(matches: &ArgMatches) {
    let hsf = matches.value_of("hsf").unwrap();
    let htf = matches.value_of("htf").unwrap();

    let gen_ctx = parse_hsf_file(hsf).unwrap();

    let multitrace = parse_htf_file(&gen_ctx,htf).unwrap();


    println!("{:?}",&multitrace);
}