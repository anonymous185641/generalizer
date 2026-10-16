use crate::petri_nets::petri_nets_to_regex::petri_nets_to_regex::PetriNetToRegexError;

pub enum TsCompositionError{
    TransformationIntoPetriNetsError,
    TransformationOfPetriNetsIntoLocalInteractionsError(PetriNetToRegexError),
    CompositionOfLocalInteractionsError,
    PetriNetParsingError,
    GeneralContextExtractionError,
    RegexConversionIntoInteractionError,
}

impl std::fmt::Display for TsCompositionError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TsCompositionError::TransformationIntoPetriNetsError => write!(f,"Error during the transformation of the transition systems into Petri nets"),
            TsCompositionError::TransformationOfPetriNetsIntoLocalInteractionsError(e) => write!(f,"Error during the transformation of the Petri nets into local interactions\n {}",e),
            TsCompositionError::CompositionOfLocalInteractionsError => write!(f,"Error during the composition of the local interactions"),
            TsCompositionError::PetriNetParsingError => write!(f,"Error during the parsing of the Petri nets"),
            TsCompositionError::GeneralContextExtractionError => write!(f,"Error during the extraction of the general context"),
            TsCompositionError::RegexConversionIntoInteractionError => write!(f,"Error during the conversion of a regular expression into local interaction"),
        }
    }
}














