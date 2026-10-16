


use std::fmt;
#[allow(dead_code)]
#[derive(Debug)]
pub enum ParsingError {
    FileFormatError(String,String),
    FileError(String),
    MatchError(String),
    // ***
    HsfSetupError(String),
    HcfSetupError(String),
    ProcessFilterError(String),
    ProcessPriorityError(String),
    // ***
    MissingMessageDeclarationError(String),
    MissingLifelineDeclarationError(String),
    MissingGateDeclarationError(String),
    MissingLifelineOrGateDeclarationError(String),
    // ***
    EmissionDefinitionError(String),
    OtherDefinitionError(String),
    // ***
    NonDisjointTraceComponents,
    IllDefinedTraceComponents(String)
}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParsingError::FileFormatError( got, expected) => {
                return write!(f, "{}", format!("expected '.{}' file and got '.{}' file", expected, got));
            },
            ParsingError::FileError(sub_e) => {
                return write!(f, "{}", format!("error while reading SD conf file : {:}", sub_e));
            },
            ParsingError::MatchError(sub_e) => {
                return write!(f, "{}", format!("error while parsing SD string : {:}", sub_e));
            },
            // ***
            ParsingError::HsfSetupError(sub_e) => {
                return write!(f, "{}", format!("error while parsing setup section of .hsf file : {:}", sub_e));
            },
            ParsingError::HcfSetupError(sub_e) => {
                return write!(f, "{}", format!("error while parsing setup section of .hcf file : {:}", sub_e));
            },
            ParsingError::ProcessFilterError(sub_e) => {
                return write!(f, "{}", format!("error while parsing filters in .hcf file : {:}", sub_e));
            },
            ParsingError::ProcessPriorityError(sub_e) => {
                return write!(f, "{}", format!("error while parsing priorities in .hcf file : {:}", sub_e));
            },
            // ***
            ParsingError::MissingMessageDeclarationError(sub_e) => {
                return write!(f, "{}", format!("error while parsing ; missing message declaration : {:}", sub_e));
            },
            ParsingError::MissingLifelineDeclarationError(sub_e) => {
                return write!(f, "{}", format!("error while parsing ; missing lifeline declaration : {:}", sub_e));
            },
            ParsingError::MissingGateDeclarationError(sub_e) => {
                return write!(f, "{}", format!("error while parsing ; missing gate declaration : {:}", sub_e));
            },
            ParsingError::MissingLifelineOrGateDeclarationError(sub_e) => {
                return write!(f, "{}", format!("error while parsing ; missing lifeline or gate declaration : {:}", sub_e));
            },
            // ***
            ParsingError::EmissionDefinitionError(sub_e) => {
                return write!(f, "{}", format!("error while parsing ; emission definition error : {:}", sub_e));
            },
            ParsingError::OtherDefinitionError(sub_e) => {
                return write!(f, "{}", format!("error while parsing ; other definition error : {:}", sub_e));
            },
            // ***
            ParsingError::NonDisjointTraceComponents => {
                return write!(f, "{}", format!("error while parsing ; non disjoint trace canals"));
            },
            ParsingError::IllDefinedTraceComponents(sub_e) => {
                return write!(f, "{}", format!("error while parsing ; ill defined trace canals : {:}", sub_e));
            }
        }
    }
}
















