use std::collections::BTreeSet;
use crate::interactions::syntax::action::{Action, ActionType};


pub type Trace = Vec<Action>;

impl Action{
    pub fn get_actions_kinds(set_of_actions : &BTreeSet<Action>) -> (i32,i32) {
        let mut num_emissions = 0;
        let mut num_receptions = 0;
        for tract in set_of_actions {
            match tract.action_type {
                ActionType::Emission => {
                    num_emissions += 1;
                },
                ActionType::Reception => {
                    num_receptions += 1;
                }
            }
        }
        return (num_emissions,num_receptions);
    }
}