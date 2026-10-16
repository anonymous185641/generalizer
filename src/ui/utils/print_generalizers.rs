use std::fs;
use crate::anti_unification::generalizer::generalizer::Generalizer;

pub fn print_generalizers(gen_vec: &Vec<Generalizer>) {

    let output_dir = "generalizer_output";
    fs::remove_dir_all(output_dir).ok();

    fs::create_dir(output_dir).ok();

    println!("========================================");
    for (ct,gen) in gen_vec.iter().enumerate() {
        println!("Generalizer {}",ct);
        println!("{}",gen);
        println!("========================================");
    }
}