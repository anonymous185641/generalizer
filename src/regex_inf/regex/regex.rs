use std::cmp::Ordering;
use std::collections::{BTreeSet, HashSet};
use std::fmt::Debug;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::Write;
use itertools::Itertools;
use is_empty::IsEmpty;
use crate::petri_nets::petri_net::place_transition::PetriNetTransitionLabelBound;
use crate::petri_nets::petri_net_io::petri_net_parsing::petri_nets_string_parser::LiteralFromString;

#[derive(Debug, Clone)]
pub enum Regex<String> {
    //General regex stuff
    Empty,
    Epsilon,
    Literal(String),
    Or(Vec<Regex<String>>),
    Concat(Vec<Regex<String>>),
    Kleene(Box<Regex<String>>),
    Plus(Box<Regex<String>>),
    Qm(Box<Regex<String>>), // Question mark
    Int(Vec<Regex<String>>),
    //SOA specific
    SRC,
    SNK
}

impl<T> PetriNetTransitionLabelBound for T
where
    T: Debug
    + Clone
    + PartialEq
    + Eq
    + Hash
    + PartialOrd
    + Ord
    + ToString
    + LiteralFromString<T>
    + IsEmpty,
{
}
impl IsEmpty for Regex<String> {
    fn is_empty(&self) -> bool {
        match self {
            Regex::Empty | Regex::Epsilon => true,
            _ => false,
        }
    }
}

impl PartialEq for Regex<String> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Regex::Empty, Regex::Empty) => true,
            (Regex::Epsilon, Regex::Epsilon) => true,
            (Regex::Literal(s1), Regex::Literal(s2)) => s1 == s2,
            (Regex::Or(set1), Regex::Or(set2)) => set1 == set2,
            (Regex::Concat(vec1), Regex::Concat(vec2)) => vec1 == vec2,
            (Regex::Kleene(boxed1), Regex::Kleene(boxed2)) => *boxed1 == *boxed2,
            (Regex::Plus(boxed1), Regex::Plus(boxed2)) => *boxed1 == *boxed2,
            (Regex::Qm(boxed1), Regex::Qm(boxed2)) => *boxed1 == *boxed2,
            (Regex::Int(set1), Regex::Int(set2)) => set1 == set2,
            (Regex::SRC, Regex::SRC) => true,
            (Regex::SNK, Regex::SNK) => true,
            _ => false,
        }
    }
}

impl Eq for Regex<String> {}

impl PartialOrd for Regex<String> { // to check
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Regex<String> { // to check
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Regex::Empty, Regex::Empty) => std::cmp::Ordering::Equal,
            (Regex::Epsilon, Regex::Epsilon) => std::cmp::Ordering::Equal,
            (Regex::Literal(s1), Regex::Literal(s2)) => s1.cmp(s2),
            (Regex::Or(set1), Regex::Or(set2)) => set1.cmp(set2),
            (Regex::Concat(vec1), Regex::Concat(vec2)) => vec1.cmp(vec2),
            (Regex::Kleene(boxed1), Regex::Kleene(boxed2)) => boxed1.cmp(boxed2),
            (Regex::Plus(boxed1), Regex::Plus(boxed2)) => boxed1.cmp(boxed2),
            (Regex::Qm(boxed1), Regex::Qm(boxed2)) => boxed1.cmp(boxed2),
            (Regex::Int(set1), Regex::Int(set2)) => set1.cmp(set2),
            (Regex::SRC, Regex::SRC) => std::cmp::Ordering::Equal,
            (Regex::SNK, Regex::SNK) => std::cmp::Ordering::Equal,
            _ => std::cmp::Ordering::Less,
        }
    }
}
impl Hash for Regex<String> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Regex::Empty => 0.hash(state),
            Regex::Epsilon => 1.hash(state),
            Regex::Literal(s) => {
                2.hash(state);
                s.hash(state);
            },
            Regex::Or(set) => {
                3.hash(state);
                for item in set {
                    item.hash(state);
                }
            },
            Regex::Concat(vec) => {
                4.hash(state);
                for item in vec {
                    item.hash(state);
                }
            },
            Regex::Kleene(boxed) => {
                5.hash(state);
                boxed.hash(state);
            },
            Regex::Plus(boxed) => {
                6.hash(state);
                boxed.hash(state);
            },
            Regex::Qm(boxed) => {
                7.hash(state);
                boxed.hash(state);
            },
            Regex::Int(set) => {
                8.hash(state);
                for item in set {
                    item.hash(state);
                }
            },
            Regex::SRC => 9.hash(state),
            Regex::SNK => 10.hash(state),
        }
    }
}


impl Regex<String> {

    pub fn concatenate(label_1: &Regex<String>, label_2: &Regex<String>)-> Regex<String>{// TO TEST
        let result = match (label_1,label_2) {
            (Regex::SRC,_) =>  Regex::SRC,
            (_,Regex::SRC) =>  Regex::SRC,
            (Regex::SNK,_) =>  Regex::SNK,
            (_,Regex::SNK) =>  Regex::SNK,
            (Regex::Empty,label) =>  label.clone(),
            (label,Regex::Empty) =>  label.clone(),
            (Regex::Concat(v),Regex::Concat(t)) =>Regex::Concat(Regex::append_and_return(v.clone(),t.clone())),
            (Regex::Concat(v),_) => Regex::Concat( Regex::extend_vector_right(&v,&label_2)),
            (_,Regex::Concat(v)) =>Regex::Concat( Regex::extend_vector_left(&v,&label_1)),
            _ => Regex::Concat(vec![label_1.clone(),label_2.clone()]),
        };

        result
    }

    pub fn concatenate_vec(mut v: Vec<Self>) -> Self {
        if v.iter().any(|r| *r == Regex::Empty) {
            return Regex::Empty;
        }

        v.retain(|r| *r != Regex::Epsilon);

        if v.is_empty() {
            Regex::Epsilon
        } else if v.len() == 1 {
            v.pop().unwrap()
        } else {
            Regex::Concat(v)
        }
    }

    pub fn plus (label: &Regex<String>)-> Regex<String>{
        match label{
            Regex::Epsilon | Regex::Empty => label.clone(),
            _ => Regex::Plus(Box::new(label.clone()))
        }

    }
    pub fn question_mark (label: &Regex<String>)-> Regex<String>{
        Regex::Qm(Box::new(label.clone()))
    }
    pub fn kleene (label: &Regex<String>)-> Regex<String>{
        match label{
            Regex::Epsilon | Regex::Empty => label.clone(),
            _ => Regex::Kleene(Box::new(label.clone()))
        }
    }

    pub fn kleene_wrap(r: Self) -> Self {
        match r {
            Regex::Empty | Regex::Epsilon => r.clone(),
            Regex::Kleene(_) => r,
            _ => Regex::Kleene(Box::new(r)),
        }
    }
    pub fn or(label_1: &Regex<String>, label_2: &Regex<String>)-> Regex<String>{// TO TEST
        let result = match (label_1,label_2) {
            (Regex::SRC,_) =>  Regex::SRC,
            (_,Regex::SRC) =>  Regex::SRC,
            (Regex::SNK,_) =>  Regex::SNK,
            (_,Regex::SNK) =>  Regex::SNK,
            (Regex::Empty,s) =>  s.clone(),
            (s,Regex::Empty) =>  s.clone(),
            (Regex::Or(v),Regex::Or(t)) => Regex::Or(Regex::union_and_return(v.clone(),t.clone())),
            (Regex::Or(v),_) =>Regex::Or(Self::extend_vector_right(v,label_2)),
            (_,Regex::Or(t)) =>Regex::Or(Self::extend_vector_left(t,label_1)),
            _ => Regex::Or(Vec::from([label_1.clone(),label_2.clone()])),
        };

        result
    }

    pub fn or_vec_retain_non_empty(mut v: Vec<Self>) -> Self {
        v.retain(|r| *r != Regex::Empty);
        if v.is_empty() {
            Regex::Empty
        } else if v.len() == 1 {
            v.pop().unwrap()
        } else {
            Regex::Or(v)
        }
    }

    pub fn interleave(label_1: &Regex<String>, label_2: &Regex<String>)-> Regex<String>{// TO TEST
        let result = match (label_1,label_2) {
            (Regex::SRC,_) =>  Regex::SRC,
            (_,Regex::SRC) =>  Regex::SRC,
            (Regex::SNK,_) =>  Regex::SNK,
            (_,Regex::SNK) =>  Regex::SNK,
            (Regex::Empty,s) =>  s.clone(),
            (s,Regex::Empty) =>  s.clone(),
            (Regex::Int(v),Regex::Int(t)) => Regex::Int(Regex::union_and_return(v.clone(),t.clone())),
            (Regex::Int(v),_) =>Regex::Int(Self::extend_vector_right(v,label_2)),
            (_,Regex::Int(t)) =>Regex::Int(Self::extend_vector_left(t,label_1)),
            _ => Regex::Int(Vec::from([label_1.clone(),label_2.clone()])),
        };

        result
    }

    pub fn sym(&self) -> usize {
        let mut set = HashSet::new();
        let mut has_src = false;
        let mut has_snk = false;
        self.collect_syms(&mut set, &mut has_src, &mut has_snk);
        set.len() + (has_src as usize) + (has_snk as usize)
    }

    fn collect_syms(&self, set: &mut HashSet<String>, has_src: &mut bool, has_snk: &mut bool) {
        match self {
            Regex::Empty | Regex::Epsilon => {}
            Regex::Literal(s) => {
                set.insert(s.clone());
            }
            Regex::Or(v) | Regex::Concat(v) | Regex::Int(v) => {
                for e in v {
                    e.collect_syms(set, has_src, has_snk);
                }
            }
            Regex::Kleene(e) | Regex::Plus(e) | Regex::Qm(e) => {
                e.collect_syms(set, has_src, has_snk);
            }
            Regex::SRC => *has_src = true,
            Regex::SNK => *has_snk = true,
        }
    }

    pub fn meta(&self) -> usize {
        match self {
            Regex::Empty | Regex::Epsilon | Regex::Literal(_) | Regex::SRC | Regex::SNK => 0,
            Regex::Or(v) => {
                let inner: usize = v.iter().map(|re| re.meta()).sum();
                if v.is_empty() {
                    0
                } else if v.len() == 1 {
                    inner + 2 // (r)
                } else {
                    inner + v.len() + 1 // (r1|r2|...|rn) -> 2 parens + (n-1) pipes
                }
            }
            Regex::Concat(v) => {
                let inner: usize = v.iter().map(|re| re.meta()).sum();
                if v.is_empty() {
                    0
                } else if v.len() == 1 {
                    inner + 2 // (r)
                } else {
                    inner + v.len() + 1 // (r1.r2. ... .rn) -> 2 parens + (n-1) dots
                }
            }
            Regex::Int(v) => {
                let inner: usize = v.iter().map(|re| re.meta()).sum();
                if v.is_empty() {
                    0
                } else if v.len() == 1 {
                    inner + 2 // (r)
                } else {
                    inner + v.len() + 1 // (r1&r2&...&rn) -> 2 parens + (n-1) ampersands
                }
            }
            Regex::Kleene(re) => 3 + re.meta(), // (r)*
            Regex::Plus(re) => 3 + re.meta(),   // (r)+
            Regex::Qm(re) => 3 + re.meta(),     // (r)?
        }
    }

    pub fn len(&self) -> f64 {
        let s = self.sym() as f64;
        let m = self.meta() as f64;
        if s + m == 0.0 {
            return 0.0;
        }
        (s + m) * (s + m).log2().ceil()
    }
    



}

