use std::fs;
use std::path::Path;
use crate::interactions::io::file_extensions::INTERACTION_FILE_EXTENSION;
use crate::interactions::io::input::error::ParsingError;
use crate::interactions::io::input::hif::interaction::parse_hif_string;
use crate::interactions::syntax::general_context::GeneralContext;
use crate::interactions::syntax::interaction::Interaction;

pub fn parse_hif_file(gen_ctx: &GeneralContext, file_path: &str) -> Result<Interaction,ParsingError> {
    let path_object = Path::new(file_path);
    let file_extension = path_object.extension().unwrap().to_str().unwrap();
    
    if file_extension != INTERACTION_FILE_EXTENSION {
        return Err(ParsingError::FileFormatError(file_extension.to_string(),INTERACTION_FILE_EXTENSION.to_string()));
    }
    
    match fs::read_to_string(file_path) {
        Ok( unparsed_hif_str) => {
            return parse_hif_string(gen_ctx,unparsed_hif_str);
        },
        Err(e)=>{
            return Err(ParsingError::FileError(e.to_string()));
        }
    }
}