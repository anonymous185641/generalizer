use std::collections::{BTreeMap, HashMap};
use std::fmt;
use crate::terms::substitution::variable::Variable;
use crate::terms::term::Term;

#[derive(Clone, PartialEq, Debug, Eq, Hash, Ord, PartialOrd)]
pub struct Substitution{
    pub map: BTreeMap<Variable, Term>,
}

/*
#[derive(Clone, PartialEq, Debug, Eq)]
pub struct Substitution{
    pub map: HashMap<Variable, Term>,
}
 */

/*
#[derive(Clone, PartialEq, Debug, Eq)]
pub struct Substitution{
    pub map: HashMap<Variable, Term>,
}
 */


impl Substitution {
    pub fn new()->Self{
        Self{ map: BTreeMap::new()}
    }

    /**/
    pub fn from_map(map: &BTreeMap<Variable,Term>) -> Substitution{
        Substitution{map: map.clone()}
    }

    pub fn dom_vars(&self) -> Vec<Variable> {
        self.map.keys().cloned().collect()
    }

    pub fn insert(&mut self, var: &Variable, term: &Term) {
        self.map.insert(var.clone(), term.clone());
    }

    pub fn get(&self, var: &Variable) -> Option<&Term> {
        self.map.get(var)
    }


    /*
    pub fn insert(&mut self, var: &Variable, term: &Term) {
        todo!()
    }
    pub fn get(&self, var: &Variable) -> Option<&Term> {
        todo!()
    }

     */


}


impl fmt::Display for Substitution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();
        let mut items: Vec<(&Variable, &Term)> = self.map.iter().collect();
        items.sort_by_key(|(v, _)| *v);
        let mut ct = items.len();

        result.push('{');
        for (variable, t) in items {
            result.push_str(&format!("{} --> {}", variable, t));

            ct -= 1;

            if ct >0{
                result.push(',');
            }
        }

        result.push('}');
        write!(f, "{}", result.trim_end())
    }
}

