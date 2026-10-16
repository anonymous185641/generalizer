/*
Copyright 2020 Erwan Mahe (github.com/erwanM974)

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

use std::fmt::Formatter;
use strum_macros::IntoStaticStr;

use crate::interactions::syntax::iat_canonize::transformation::transfofunc::alt_dedup::alt_dedup_equal::transfo_alt_deduplicate;
use crate::interactions::syntax::iat_canonize::transformation::transfofunc::defactorize::{transfo_defactorize_left, transfo_defactorize_right};
use crate::interactions::syntax::iat_canonize::transformation::transfofunc::factorize::factorize_par::transfo_factorize_par;
use crate::interactions::syntax::iat_canonize::transformation::transfofunc::factorize::factorize_prefix::transfo_factorize_prefix_seq;
use crate::interactions::syntax::iat_canonize::transformation::transfofunc::factorize::factorize_suffix::transfo_factorize_suffix_seq;
use crate::interactions::syntax::iat_canonize::transformation::transfofunc::flush::transfo_flush_right;
use crate::interactions::syntax::iat_canonize::transformation::transfofunc::invert::{transfo_invert_alt_sorted, transfo_invert_par_sorted, transfo_invert_tensor_sorted};
use crate::interactions::syntax::iat_canonize::transformation::transfofunc::loop_dedup::loop_dedup::transfo_loops_deduplicate;
use crate::interactions::syntax::iat_canonize::transformation::transfofunc::loop_simpl::transfo_loop_empty_simpl;
use crate::interactions::syntax::iat_canonize::transformation::transfofunc::simpl::transfo_simpl;
use crate::interactions::syntax::interaction::Interaction;


#[derive(IntoStaticStr,Clone, PartialEq, Debug, Eq, Hash)]
pub enum InteractionTransformationKind {
    Simpl,
    FlushRight,
    InvertAlt,
    InvertPar,
    InvertTensor,
    AltDeduplicate,
    LoopSDeduplicate,
    LoopSimpl,
    FactorizePrefixSeq,
    FactorizeCommutativePar,
    FactorizeSuffixSeq,
    DeFactorizeLeft,
    DeFactorizeRight,

    // ***
}

impl std::fmt::Display for InteractionTransformationKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let as_static_str : &'static str = self.into();
        write!(f,"{}", as_static_str)
    }
}


impl InteractionTransformationKind {
    pub fn to_string(&self) -> String {
        let as_static_str : &'static str = self.into();
        return as_static_str.to_string();
    }
    pub fn get_transformation(&self) -> fn(&Interaction) -> Vec<Interaction> {
        match self {
            InteractionTransformationKind::Simpl => {
                return transfo_simpl;
            }
            InteractionTransformationKind::FlushRight => {
                return transfo_flush_right;
            },
            InteractionTransformationKind::InvertAlt => {
                return transfo_invert_alt_sorted;
            },
            InteractionTransformationKind::InvertPar => {
                return transfo_invert_par_sorted;
            },
            InteractionTransformationKind::InvertTensor => {
                return transfo_invert_tensor_sorted;
            },
            InteractionTransformationKind::AltDeduplicate => {
                return transfo_alt_deduplicate;
            },
            InteractionTransformationKind::LoopSDeduplicate => {
                return transfo_loops_deduplicate;
            },
            InteractionTransformationKind::LoopSimpl => {
                return transfo_loop_empty_simpl;
            },

            InteractionTransformationKind::FactorizePrefixSeq => {
                return transfo_factorize_prefix_seq;
            },
            InteractionTransformationKind::FactorizeSuffixSeq => {
                return transfo_factorize_suffix_seq;
            },
            InteractionTransformationKind::FactorizeCommutativePar => {
                return transfo_factorize_par;
            },
            InteractionTransformationKind::DeFactorizeLeft => {
                return transfo_defactorize_left;
            },
            InteractionTransformationKind::DeFactorizeRight => {
                return transfo_defactorize_right;
            },
            /*

            InteractionTransformationKind::ParToSeq => {
                return transfo_par_to_seq;
            },

             */
        }
    }
}

