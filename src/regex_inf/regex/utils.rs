use std::collections::HashSet;
use std::hash::Hash;
use crate::regex_inf::regex::regex::Regex;

impl Regex<String>{

    pub fn binom(n: usize, k: usize) -> u128 {
        if k > n { return 0; }

        let k = k.min(n - k);

        let mut res = 1u128;
        for i in 0..k {
            res = res * (n - i) as u128 / (i + 1) as u128;
        }
        res
    }

    pub fn extract_alphabet(re: &Regex<String>, set: &mut HashSet<String>) {
        match re {
            Regex::Literal(s) => { set.insert(s.clone()); }
            Regex::Or(v) | Regex::Concat(v) | Regex::Int(v) => {
                for e in v { Self::extract_alphabet(e, set); }
            }
            Regex::Kleene(e) | Regex::Plus(e) | Regex::Qm(e) => {
                Self::extract_alphabet(e, set);
            }
            _ => {}
        }
    }

    pub fn append_and_return<T: Clone>(v: Vec<T>,t: Vec<T>)-> Vec<T>{
        let mut ret = v.clone();
        let other = t.clone();
        ret.extend(other);
        ret
    }
    pub fn union_and_return<T: Eq + Hash+Clone>(v: Vec<T>,t: Vec<T>)-> Vec<T>{

        let mut result = v.clone();
        result.append(&mut t.clone());

        result
    }
    pub fn extend_vector_right<T:Clone>(input_vector: &Vec<T>, val: &T )-> Vec<T>{
        // This method adds a value at the end of a vector and returns the result
        let mut new_vec: Vec<T> = Vec::new();
        for element in input_vector.iter(){
            new_vec.push(element.clone());
        }
        new_vec.push(val.clone());

        new_vec
    }
    pub fn extend_vector_left<T:Clone>(input_vector: &Vec<T>, val: &T )-> Vec<T>{
        // This method adds a value at the end of a vector and returns the result
        let mut new_vec: Vec<T> = Vec::new();
        new_vec.push(val.clone());
        for element in input_vector{
            new_vec.push(element.clone());
        }
        new_vec
    }

    pub fn extend_vec<T: Eq + PartialEq+ Hash+Clone>(input_tree: &HashSet<T>,val: &T)-> HashSet<T>{
        let mut new_tree: HashSet<T> = HashSet::new();

        for element in input_tree{
            new_tree.insert(element.clone());
        }
        new_tree.insert(val.clone());

        new_tree
    }

    pub fn remove_epsilon_and_empty(v: &Vec<Regex<String>>)->Vec<Regex<String>>{
        let mut ret = Vec::new();
        for el in v{
            if *el!=Regex::Epsilon && *el!= Regex::Empty{
                ret.push(el.clone());
            }
        }
        ret
    }

    pub fn remove_empty(v: &Vec<Regex<String>>)->Vec<Regex<String>>{
        let mut ret = Vec::new();
        for el in v{
            if *el!= Regex::Empty{
                ret.push(el.clone());
            }
        }
        ret
    }
    pub fn is_head_kleene(&self)->bool{
        match self{
            Regex::Plus(_)|Regex::Kleene(_)=>{
                true
            },
            _ => false,
        }
    }
    pub fn contains_kleene(vec:&Vec<Regex<String>>)->bool{
        for el in vec{
            match el{
                Regex::Plus(_)|Regex::Kleene(_)=>{
                    return true;
                },
                _ => continue,
            }
        }
        false
    }
}