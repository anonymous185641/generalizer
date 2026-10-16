
use std::fmt::Formatter;
use strum_macros::IntoStaticStr;
use crate::anti_unification::configuration::configuration::Configuration;
use crate::anti_unification::generalization_process::generalization_transformations::transfofunc::coreg_rules::decompose_a_coreg::{can_apply_decompose_a_coreg, transfo_decompose_a_coreg};
use crate::anti_unification::generalization_process::generalization_transformations::transfofunc::coreg_rules::decompose_ac_coreg::{can_apply_decompose_ac_coreg, transfo_decompose_ac_coreg};
use crate::anti_unification::generalization_process::generalization_transformations::transfofunc::decompose::{can_apply_decompose, transfo_decompose};
use crate::anti_unification::generalization_process::generalization_transformations::transfofunc::decompose_a::{can_apply_decompose_a, transfo_decompose_a};
use crate::anti_unification::generalization_process::generalization_transformations::transfofunc::decompose_ac::{can_apply_decompose_ac, transfo_decompose_ac};
use crate::anti_unification::generalization_process::generalization_transformations::transfofunc::decompose_c::{can_apply_decompose_c, transfo_decompose_c};
use crate::anti_unification::generalization_process::generalization_transformations::transfofunc::fail::{can_apply_fail, transfo_fail};
use crate::anti_unification::generalization_process::generalization_transformations::transfofunc::recover::{can_apply_recover, transfo_recover};
use crate::anti_unification::generalization_process::generalization_transformations::transfofunc::sc_solve::{can_apply_sc_solve, transfo_sc_solve};
use crate::anti_unification::generalization_process::generalization_transformations::transfofunc::seq_special_commutativity::decompose_seq::{can_apply_decompose_seq, transfo_decompose_seq};
use crate::anti_unification::generalization_process::generalization_transformations::transfofunc::solve::{can_apply_solve, transfo_solve};
use crate::anti_unification::generalization_process::generalization_transformations::transfofunc::unit_element_rules::expand_same::expand_u_same_left_decompose::{can_apply_expand_u_same_left_decompose, transfo_expand_u_same_left_decompose};
use crate::anti_unification::generalization_process::generalization_transformations::transfofunc::unit_element_rules::expand_same::expand_u_same_right_decompose::{can_apply_expand_u_same_right_decompose, transfo_expand_u_same_right_decompose};
use crate::anti_unification::generalization_process::generalization_transformations::transfofunc::unit_element_rules::expand_u::expand_u_left_decompose::{can_apply_expand_u_left_decompose, transfo_expand_u_left_decompose};
use crate::anti_unification::generalization_process::generalization_transformations::transfofunc::unit_element_rules::expand_u::expand_u_right_decompose::{can_apply_expand_u_right_decompose, transfo_expand_u_right_decompose};
use crate::anti_unification::generalization_process::generalization_transformations::transfofunc::variant::variant_rule::{can_apply_variant, transfo_variant};

#[derive(IntoStaticStr,Clone, PartialEq, Debug, Eq, Hash)]
pub enum ConfigurationTransformationKind {
    Decompose,
    Solve,
    Recover,
    ScSolve,
    Fail,
    DecomposeA,
    DecomposeAC,
    DecomposeC,
    DecomposeCSeq,
    DecomposeAC_Coreg,
    DecomposeA_Coreg,
    ExpandULeftDecompose,
    ExpandURightDecompose,
    ExpandUSameLeftDecompose,
    ExpandUSameRightDecompose,
    Variant
}

impl std::fmt::Display for ConfigurationTransformationKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let as_static_str : &'static str = self.into();
        write!(f,"{}", as_static_str)
    }
}


impl ConfigurationTransformationKind {
    pub fn to_string(&self) -> String {
        let as_static_str : &'static str = self.into();
        return as_static_str.to_string();
    }

    pub fn get_transformation_precondition(&self)-> fn(&Configuration) -> bool {
        match self{
            ConfigurationTransformationKind::Fail => {
                return can_apply_fail;
            },
            ConfigurationTransformationKind::Decompose => {
                return can_apply_decompose;
            },
            ConfigurationTransformationKind::Solve => {
                return can_apply_solve;
            },
            ConfigurationTransformationKind::Recover => {
                return can_apply_recover;
            },
            ConfigurationTransformationKind::ScSolve => {
                return can_apply_sc_solve;
            },
            ConfigurationTransformationKind::DecomposeA => {
                return can_apply_decompose_a;
            },
            ConfigurationTransformationKind::DecomposeAC => {
                return can_apply_decompose_ac;
            },
            ConfigurationTransformationKind::DecomposeC => {
                return can_apply_decompose_c;
            },
            ConfigurationTransformationKind::DecomposeCSeq => {
                return can_apply_decompose_seq;
            },
            ConfigurationTransformationKind::DecomposeAC_Coreg => {
                return can_apply_decompose_ac_coreg;
            },
            ConfigurationTransformationKind::DecomposeA_Coreg => {
                return can_apply_decompose_a_coreg;
            }
            ConfigurationTransformationKind::ExpandULeftDecompose => {
                return can_apply_expand_u_left_decompose;
            },
            ConfigurationTransformationKind::ExpandURightDecompose => {
                return can_apply_expand_u_right_decompose;
            },
            ConfigurationTransformationKind::ExpandUSameLeftDecompose => {
                return can_apply_expand_u_same_left_decompose
            },
            ConfigurationTransformationKind::ExpandUSameRightDecompose => {
                return can_apply_expand_u_same_right_decompose
            },
            ConfigurationTransformationKind::Variant => {
                return can_apply_variant;
            }
        }
    }
    pub fn get_transformation(&self) -> fn(&Configuration) -> Vec<Configuration> {
        match self {
            ConfigurationTransformationKind::Decompose => {
                return transfo_decompose;
            }
            ConfigurationTransformationKind::Solve => {
                return transfo_solve;
            },
            ConfigurationTransformationKind::Recover => {
                return transfo_recover;
            },
            ConfigurationTransformationKind::ScSolve => {
                return transfo_sc_solve;
            },
            ConfigurationTransformationKind::Fail => {
                return transfo_fail;
            },
            ConfigurationTransformationKind::DecomposeA => {
                return transfo_decompose_a;
            },
            ConfigurationTransformationKind::DecomposeAC => {
                return transfo_decompose_ac;
            },
            ConfigurationTransformationKind::DecomposeC => {
                return transfo_decompose_c;
            },
            ConfigurationTransformationKind::DecomposeCSeq => {
                return transfo_decompose_seq;
            },
            ConfigurationTransformationKind::DecomposeAC_Coreg => {
                return transfo_decompose_ac_coreg;
            },
            ConfigurationTransformationKind::DecomposeA_Coreg => {
                return transfo_decompose_a_coreg;
            }
            ConfigurationTransformationKind::ExpandULeftDecompose => {
                return transfo_expand_u_left_decompose;
            },
            ConfigurationTransformationKind::ExpandURightDecompose => {
                return transfo_expand_u_right_decompose;
            },
            ConfigurationTransformationKind::ExpandUSameLeftDecompose => {
                return transfo_expand_u_same_left_decompose
            },
            ConfigurationTransformationKind::ExpandUSameRightDecompose => {
                return transfo_expand_u_same_right_decompose
            },
            ConfigurationTransformationKind::Variant=>{
                return transfo_variant;
            }
        }
    }
}

