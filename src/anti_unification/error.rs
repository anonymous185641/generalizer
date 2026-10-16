#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum ConfigurationError {
    SolveFailed,
    ConstrainedGeneralisationFailed,
    NonGroundTerm,
    InvalidRuleApplication,
    RuleApplicationError,
    UnknownRule,
    TimedOut,
}

impl std::fmt::Display for ConfigurationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigurationError::SolveFailed => write!(f, "Solve-failed"),
            ConfigurationError::ConstrainedGeneralisationFailed => write!(f, "Constrained generalisation failed"),
            ConfigurationError::NonGroundTerm => write!(f, "Generalization of non-ground term"),
            ConfigurationError::InvalidRuleApplication => write!(f, "Invalid rule application"),
            ConfigurationError::RuleApplicationError => write!(f, "Rule application error"),
            ConfigurationError::UnknownRule => write!(f, "Trying to apply an unknown rule"),
            ConfigurationError::TimedOut => write!(f, "Timed out"),
        }
    }
}

impl std::error::Error for ConfigurationError {}