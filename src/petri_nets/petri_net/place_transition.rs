use std::fmt::Debug;
use std::hash::Hash;
use is_empty::IsEmpty;
use crate::global_counter::counter::fresh_number;
use crate::petri_nets::petri_net_io::petri_net_parsing::petri_nets_string_parser::LiteralFromString;


pub trait PetriNetTransitionLabelBound:
Debug
+ Clone
+ PartialEq
+ Eq
+ Hash
+ PartialOrd
+ Ord
+ ToString
+ LiteralFromString<Self>
+ IsEmpty
{
}



#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PlaceId(pub usize);

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TransitionId(pub usize);

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PetriPlace{
    pub id: PlaceId,
    pub label: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PetriTransition<T: PetriNetTransitionLabelBound>{
    pub id: TransitionId,
    pub label: T,
}


impl PetriPlace{
    pub fn new(id: &PlaceId, label: &String) -> Self{
        Self{
            id: id.clone(),
            label: label.clone()
        }
    }
    pub fn new_from_name(label: &String) ->Self{
        let id = PlaceId(fresh_number());
        Self::new(&id, label)
    }
    pub fn new_from_nothing()->Self{
        let id = PlaceId(fresh_number());
        let label = format!("p{}", id.0);
        Self::new(&id,&label)
    }
}


impl<T: PetriNetTransitionLabelBound> PetriTransition<T> {
    pub fn new(id: &TransitionId, label: &T) -> Self{
        Self{
            id: id.clone(),
            label: label.clone()
        }
    }
    pub fn new_from_name(label: &T) -> Self{
        let id = TransitionId(fresh_number());
        Self::new(&id, label)
    }
}