

use std::collections::BTreeSet;
use std::iter::FromIterator;


use crate::interactions::syntax::error::CoreError;

#[derive(Clone, PartialEq, Debug)]
pub struct GeneralContext {
    lf_names : Vec<String>,
    ms_names : Vec<String>
}



impl GeneralContext {

    pub fn new() -> GeneralContext {
        return GeneralContext {
            lf_names: Vec::new(),
            ms_names: Vec::new()
        }
    }

    // ********** ********** ********** ********** ********** ********** **********
    // ********** ********** ********** ********** ********** ********** **********
    // ********** ********** ********** ********** ********** ********** **********

    pub fn add_lf(&mut self, lf_name : String) -> usize {
        match self.get_lf_id(&lf_name) {
            None => {
                self.lf_names.push(lf_name);
                return self.lf_names.len() - 1;
            },
            Some(lf_id) => {
                return lf_id;
            }
        }
    }

    pub fn add_msg(&mut self, ms_name : String) -> usize {
        match self.get_ms_id(&ms_name) {
            None => {
                self.ms_names.push(ms_name);
                return self.ms_names.len() - 1;
            },
            Some(ms_id) => {
                return ms_id;
            }
        }
    }
    

    // ********** ********** ********** ********** ********** ********** **********
    // ********** ********** ********** ********** ********** ********** **********
    // ********** ********** ********** ********** ********** ********** **********

    pub fn get_lf_id(&self, lf_name : &str) -> Option<usize> {
        return self.lf_names.iter().position(|r| r == lf_name);
    }

    pub fn get_ms_id(&self, ms_name : &str) -> Option<usize> {
        return self.ms_names.iter().position(|n| n == ms_name);
    }
    

    // ********** ********** ********** ********** ********** ********** **********
    // ********** ********** ********** ********** ********** ********** **********
    // ********** ********** ********** ********** ********** ********** **********
    #[allow(dead_code)]
    pub fn get_lf_num(&self) -> usize {
        return self.lf_names.len();
    }

    #[allow(dead_code)]
    pub fn get_ms_num(&self) -> usize {
        return self.ms_names.len();
    }

    // ********** ********** ********** ********** ********** ********** **********
    // ********** ********** ********** ********** ********** ********** **********
    // ********** ********** ********** ********** ********** ********** **********
    #[allow(dead_code)]
    pub fn get_all_lfs_ids(&self) -> BTreeSet<usize> {
        return BTreeSet::from_iter(0..self.get_lf_num() );
    }

    // ********** ********** ********** ********** ********** ********** **********
    // ********** ********** ********** ********** ********** ********** **********
    // ********** ********** ********** ********** ********** ********** **********

    pub fn get_lf_name(&self, lf_id : usize) -> Result<String,CoreError> {
        match self.lf_names.get(lf_id) {
            None => {
                return Err( CoreError::UnknownLifeline(lf_id) );
            },
            Some( got_str ) => {
                return Ok( got_str.to_string() );
            }
        }
    }

    pub fn get_ms_name(&self, ms_id : usize) -> Result<String,CoreError> {
        match self.ms_names.get(ms_id) {
            None => {
                return Err( CoreError::UnknownMessage(ms_id) );
            },
            Some( ms_name ) => {
                return Ok( ms_name.to_string() );
            }
        }
    }
    
}
