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



use std::fmt;
use std::time::Instant;
use graph_process_manager_core::handler::filter::AbstractFilter;
use crate::anti_unification::generalization_process::generalization::filter::elim::GeneralizationFilterEliminationKind;

pub struct GeneralizationFilterCriterion {}

impl fmt::Display for GeneralizationFilterCriterion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"")
    }
}


pub enum GeneralizationFilter {
    Timeout((Instant,usize))
}

impl fmt::Display for GeneralizationFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GeneralizationFilter::Timeout((_,t)) => write!(f,"Timeout={}s",t),
        }
        //write!(f,"")
    }
}

impl AbstractFilter<GeneralizationFilterCriterion, GeneralizationFilterEliminationKind>  for GeneralizationFilter {

    fn apply_filter(&self,
                    _depth: u32,
                    _node_counter: u32,
                    _criterion: &GeneralizationFilterCriterion) -> Option<GeneralizationFilterEliminationKind> {

        match self {
            GeneralizationFilter::Timeout((i,t))=>{
                if i.elapsed().as_secs() as usize > *t{
                    println!("\n=====Timeout reached====\n");
                    return Some(GeneralizationFilterEliminationKind::TimoutReached);
                }
            }
        }
        None
    }

}

