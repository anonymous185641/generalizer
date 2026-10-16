use std::fs;
use std::path::Path;
use crate::interactions::io::file_extensions::SIGNATURE_FILE_EXTENSION;
use crate::interactions::io::input::error::ParsingError;
use crate::interactions::io::input::hsf::implem::parse_hsf_string;
use crate::interactions::syntax::general_context::GeneralContext;


pub fn parse_hsf_file(file_path: &str) -> Result<GeneralContext,ParsingError> {
    let path_object = Path::new(file_path);
    
    let file_extension :&str = path_object.extension().unwrap().to_str().unwrap();
    
    if file_extension != SIGNATURE_FILE_EXTENSION {
        return Err( ParsingError::FileFormatError(file_extension.to_string(),SIGNATURE_FILE_EXTENSION.to_string()));
    }
    
    match fs::read_to_string(file_path) {
        Ok(unparsed_hsf_str) => {
            return parse_hsf_string(unparsed_hsf_str);
        },
        Err(e) => {
            return Err(ParsingError::FileError(e.to_string()));
        }
    }
}