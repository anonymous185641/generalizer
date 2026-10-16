
use std::fmt::Formatter;
use strum_macros::IntoStaticStr;
use crate::anti_unification::configuration::configuration::Configuration;
use crate::matching::matching_ac::matching_configuration::matching_configuration::MatchingConfiguration;
use crate::matching::matching_ac::matching_process::matching_transformation::transfofunc::clash::{can_apply_clash, transfo_clash};
use crate::matching::matching_ac::matching_process::matching_transformation::transfofunc::dec::{can_apply_m_decompose, transfo_m_decompose};
use crate::matching::matching_ac::matching_process::matching_transformation::transfofunc::solve::{can_apply_m_solve, tranfo_m_solve};

#[derive(IntoStaticStr,Clone, PartialEq, Debug, Eq, Hash)]
pub enum MatchingConfigurationTransformationKind {
    MDecompose,
    MSolve,
    Clash,
}

impl std::fmt::Display for MatchingConfigurationTransformationKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let as_static_str : &'static str = self.into();
        write!(f,"{}", as_static_str)
    }
}


impl MatchingConfigurationTransformationKind {
    pub fn to_string(&self) -> String {
        let as_static_str : &'static str = self.into();
        return as_static_str.to_string();
    }

    pub fn get_transformation_precondition(&self)-> fn(&MatchingConfiguration) -> bool {
        match self{
            MatchingConfigurationTransformationKind::MDecompose => {
                return can_apply_m_decompose;
            },
            MatchingConfigurationTransformationKind::MSolve =>{
                return can_apply_m_solve;
            },
            MatchingConfigurationTransformationKind::Clash => {
                return can_apply_clash;
            }
        }
    }
    pub fn get_transformation(&self) -> fn(&MatchingConfiguration) -> Vec<MatchingConfiguration> {
        match self {
            MatchingConfigurationTransformationKind::MDecompose => {
                return transfo_m_decompose;
            },
            MatchingConfigurationTransformationKind::MSolve =>{
                return tranfo_m_solve;
            },
            MatchingConfigurationTransformationKind::Clash=>{
                return transfo_clash;
            }
        }
    }
}

