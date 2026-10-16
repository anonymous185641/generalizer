

#[derive(Debug, Clone,PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct State(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Trans(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TsArc(pub State,pub Trans,pub State);














