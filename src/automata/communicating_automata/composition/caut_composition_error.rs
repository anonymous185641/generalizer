use crate::petri_nets::petri_nets_to_regex::petri_nets_to_regex::PetriNetToRegexError;

pub enum CommunicatingAutomataCompositionError {
    TransformationIntoPetriNetsError,
    TransformationOfPetriNetsIntoLocalInteractionsError(PetriNetToRegexError),
    CompositionOfLocalInteractionsError(Option<(usize,usize)>),//For the benchmark, we return the range of size of the local interactions
    PetriNetParsingError,
    GeneralContextExtractionError,
    RegexSyntesisError,
    RegexConversionIntoInteractionError,
    SynchronousProductError,
}

impl std::fmt::Display for CommunicatingAutomataCompositionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommunicatingAutomataCompositionError::TransformationIntoPetriNetsError => write!(f, "Error during the transformation of the transition systems into Petri nets"),
            CommunicatingAutomataCompositionError::TransformationOfPetriNetsIntoLocalInteractionsError(e) => write!(f, "Error during the transformation of the Petri nets into local interactions\n {}", e),
            CommunicatingAutomataCompositionError::CompositionOfLocalInteractionsError(_) => write!(f, "Error during the composition of the local interactions"),
            CommunicatingAutomataCompositionError::PetriNetParsingError => write!(f, "Error during the parsing of the Petri nets"),
            CommunicatingAutomataCompositionError::GeneralContextExtractionError => write!(f, "Error during the extraction of the general context"),
            CommunicatingAutomataCompositionError::RegexConversionIntoInteractionError => write!(f, "Error during the conversion of a regular expression into local interaction"),
            CommunicatingAutomataCompositionError::SynchronousProductError => write!(f, "Error during the synchronous product of the automata"),
            CommunicatingAutomataCompositionError::RegexSyntesisError => write!(f, "Error during the synthesis of a regular expression"),
        }
    }
}














