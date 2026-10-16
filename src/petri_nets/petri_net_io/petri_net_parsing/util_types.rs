

#[derive(Debug, Clone,PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Place(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Trans(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PlaceTransArc(pub Place,pub Trans);

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TransPlaceArc(pub Trans,pub Place);



pub fn retrieve_transition_prefix(s: &str) -> &str {
    let temp =s;
    match s.rsplit_once('/') {
        Some((prefix, suffix)) if suffix.parse::<usize>().is_ok() => prefix,
        _ => temp,
    }
}


















