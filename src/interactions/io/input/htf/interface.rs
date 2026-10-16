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




use std::fs;
use std::path::Path;
use std::collections::BTreeSet;
use maplit::btreeset;


#[allow(unused_imports)]
use pest::Parser;
use crate::interactions::io::file_extensions::HIBOU_TRACE_FILE_EXTENSION;
use crate::interactions::io::input::error::ParsingError;
use crate::interactions::io::input::htf::implem::multitrace_from_text;
use crate::interactions::io::input::htf::parser::{HtfParser, Rule};
use crate::interactions::io::input::htf::trace::trace_element_from_pair;
use crate::interactions::syntax::action::Action;
use crate::interactions::syntax::colocalizations::CoLocalizations;
use crate::interactions::syntax::general_context::GeneralContext;
use crate::interactions::syntax::traces::multitrace::MultiTrace;



pub fn parse_htf_file(gen_ctx : &GeneralContext,
                      file_path : &str) -> Result<(CoLocalizations,MultiTrace),ParsingError> {
    let path_object = Path::new(file_path);
    let file_extension : &str = path_object.extension().unwrap().to_str().unwrap();
    if file_extension != HIBOU_TRACE_FILE_EXTENSION {
        return Err( ParsingError::FileFormatError(file_extension.to_string(),HIBOU_TRACE_FILE_EXTENSION.to_string()));
    }
    // ***
    match fs::read_to_string(file_path) {
        Ok( unparsed_htf_str ) => {
            return multitrace_from_text(gen_ctx,&unparsed_htf_str);
        },
        Err(e) => {
            return Err( ParsingError::FileError(e.to_string()) );
        }
    }
}





/*
pub fn multi_action_from_text(gen_ctx : &GeneralContext,
                            multiact_str : &String) -> Result<BTreeSet<Action>,ParsingError> {
    match HtfParser::parse(Rule::TRACE_SEQUENCE_elt, multiact_str) {
        Err(e) => {
            return Err( ParsingError::MatchError(e.to_string()) );
        },
        Ok( ref mut content ) => {
            let trace_elt_pair = content.next().unwrap();
            return trace_element_from_pair(gen_ctx,trace_elt_pair,&btreeset!{},&mut btreeset!{}, true);
        }
    }
}

 */
