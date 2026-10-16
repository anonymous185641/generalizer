#[derive(Debug, Clone)]
pub enum CompositionError {
    CompositionFailure,
    CompositionTimeout,
    UniqueGatePropertyUnsatisfied,
    MergeFailure,
    TimedOut,
}



impl std::fmt::Display for CompositionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompositionError::CompositionFailure => write!(f, "Composition failure"),
            CompositionError::CompositionTimeout => write!(f, "Composition timeout"),
            CompositionError::UniqueGatePropertyUnsatisfied => write!(f, "Unique gate property unsatisfied"),
            CompositionError::MergeFailure => write!(f, "Failed to merge gates"),
            CompositionError::TimedOut => write!(f, "Timed out"),
        }
    }
}