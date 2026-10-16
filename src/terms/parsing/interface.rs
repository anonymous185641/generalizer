use std::fs;
use std::path::Path;
use crate::terms::function::Signature;
use crate::terms::parsing::string_parser::parse_string;
use crate::terms::parsing::term_parser::parse_term;
use crate::terms::term::Term;

pub fn parse_file(file_path: &str) -> Result<(Signature, Term, Term),String> {
    let path = Path::new(file_path);

    let file_extension = path.extension().unwrap().to_str().unwrap();

    if file_extension != "txt" {
        return Err(format!("File extension is not .txt: {}", file_path));
    }

    match fs::read_to_string(path) {
        Ok(unparsed_string)=>{

             let (sig,t1,t2) = parse_string(&unparsed_string)?;


            let term1 = parse_term(&sig,&t1)?;
            let term2 = parse_term(&sig,&t2)?;
            
            Ok((sig,term1,term2))

        },
        Err(_)=>{

            Err(format!("File error: {}", file_path))
        }
    }
}