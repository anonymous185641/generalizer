
use crate::ui::cimit_cli::cimit_cli;

mod anti_unification;
mod global_counter;
mod interactions;
mod terms;
mod ui;
mod utils;
mod matching;
mod regex_inf;
mod interaction_mining;
mod automata;
mod petri_nets;

fn main() {

    cimit_cli();
}
