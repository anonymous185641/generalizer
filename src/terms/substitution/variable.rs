use std::fmt;
use crate::global_counter::counter::fresh_number;

#[derive(Clone, PartialEq, Debug, Eq, Hash, PartialOrd, Ord)]
pub struct Variable {
    pub label: String,
    pub id: usize,
}



impl Variable {
    pub fn new(label: String, id: usize) -> Variable {
        Self{label, id}
    }
    pub fn fresh_variable() -> Variable {
        Self{label: "x".to_string(), id: fresh_number()}
    }

    pub fn x_0()->Variable{
        Self{label: "x".to_string(), id: 0}
    }
    pub fn name(&self)->String{
        format!("{}{}", self.label, self.id)
    }
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        //write!(f, "#{}", self.id)
        write!(f, "{}", self.name())
    }
}



