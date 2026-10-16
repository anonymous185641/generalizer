use std::collections::BTreeSet;
use crate::anti_unification::configuration::configuration::Configuration;
use crate::anti_unification::generalization_process::generalization_transformations::transfofunc::seq_special_commutativity::decompose_seq::seq_comm_dec_groups;
use crate::interactions::syntax::operators::Operator;
use crate::interactions::syntax::operators::Operator::Coreg;
use crate::terms::function::{Axioms, Function, FunctionSignature};
use crate::terms::substitution::substitution::Substitution;
use crate::terms::term::Term;
use crate::utils::combinatorics::Combinatorics;
use crate::utils::misc::have_common_element;

pub fn can_apply_decompose_ac_coreg(config : &Configuration) -> bool {

    let aut = config.active[0].clone();

    if aut.t1.is_head_coreg() && aut.t2.is_head_coreg(){

        if aut.t1.head() == aut.t2.head() && aut.t1.head().eq("par") {//If Par on both sides, apply the Decompose_AC transformation instead
            return false;
        }

        return true;
    }

    false
}




pub fn transfo_decompose_ac_coreg(config : &Configuration) -> Vec<Configuration> {
    let mut return_configurations = Vec::new();

    let mut active = config.active.clone();

    //Since this is decompose, we remove the concerned AUT
    let aut = active.remove(0);

    match (aut.t1.clone().assoc_coreg_flatten(), aut.t2.clone().assoc_coreg_flatten()) {
        (Term::Function(f1), Term::Function(f2)) => {

            for (t1,t1_prime,t2,t2_prime) in ac_coreg_comm_dec_groups(&f1.signature, &f2.signature, &f1.args, &f2.args) {
                let mut new_active = active.clone();
                let new_store = config.store.clone();
                let mut new_sub = config.sub.clone();

                let (x1, x2, aut1, aut2) = config.with_fresh_aut_pair(
                    t1, t1_prime, t2, t2_prime
                );

                new_active.insert(0,aut1.clone());
                new_active.insert(0,aut2.clone());

                let mut sub = Substitution::new();
                let sub_term_args = vec![Term::Variable(x1), Term::Variable(x2)];

                let lfs_t1 = aut.t1.lifelines_ids();
                let lfs_t2 = aut.t2.lifelines_ids();
                let coreg1 = CoregType::parse_from_string(&f1.signature.name).unwrap();
                let coreg2 = CoregType::parse_from_string(&f2.signature.name).unwrap();
                let new_coreg = CoregType::merge_coreg(&coreg1,&coreg2,&lfs_t1,&lfs_t2);

                sub.insert(&aut.x, &Term::Function(Function::new(&new_coreg.to_function_signature(), &sub_term_args)));
                new_sub.push(sub);

                //let conf = Configuration::new(new_active, new_store,new_sub,config.x0.clone(),config.update_history("Decompose_AC"));

                let conf = Configuration::create_new_config(new_active, new_store, new_sub, config.x0.clone());
                return_configurations.push(conf);
            }

            return return_configurations;
        },
        _ => panic!("Error: decompose_coreg only works on terms with coreg heads")
    }
}

pub fn ac_coreg_comm_dec_groups(sig1: &FunctionSignature, sig2: &FunctionSignature, args1: &Vec<Term>, args2: &Vec<Term>) -> Vec<(Term, Term, Term, Term)> {
    let mut quadruples = Vec::new();

    let coreg1 = CoregType::parse_from_string(&sig1.name).unwrap();
    let coreg2 = CoregType::parse_from_string(&sig2.name).unwrap();


    //Left
    for i in 0..args1.len()-1 {
        let t1 = Term::assoc_wrap(sig1,&Vec::from(&args1[0..=i]));
        let t2 = Term::assoc_wrap(sig1,&Vec::from(&args1[i+1..]));

        let n = args2.len();
        for subset_size in 1..=n - 1 {
            let indices: Vec<usize> = (0..n).collect();
            let combos = Combinatorics::combinations(&indices, subset_size);

            for combo in combos {
                let (vec3, vec4) = Combinatorics::split_vector(args2, &combo);
                let t1_prime = Term::assoc_wrap(sig2, &vec3);
                let t2_prime = Term::assoc_wrap(sig2, &vec4);

                if !have_common_element(&t1_prime.lifelines_ids(),&t2_prime.lifelines_ids())
                || CoregType::all_lfs_in_coreg_domain(&coreg2,&t1_prime.lifelines_ids(),&t2_prime.lifelines_ids()){
                    quadruples.push((t1.clone(), t1_prime, t2.clone(), t2_prime));
                }

            }
        }
    }

    //Right
    for i in 0..args2.len()-1 {
        let t1_prime = Term::assoc_wrap(sig2,&Vec::from(&args2[0..=i]));
        let t2_prime = Term::assoc_wrap(sig2,&Vec::from(&args2[i+1..]));

        let n = args1.len();
        for subset_size in 1..=n - 1 {
            let indices: Vec<usize> = (0..n).collect();
            let combos = Combinatorics::combinations(&indices, subset_size);

            for combo in combos {
                let (vec1, vec2) = Combinatorics::split_vector(args1, &combo);

                let t1 = Term::assoc_wrap(sig1, &vec1);//.to_associative_form();
                let t2 = Term::assoc_wrap(sig1, &vec2);//.to_associative_form();

                if !have_common_element(&t1.lifelines_ids(),&t2.lifelines_ids())
                || CoregType::all_lfs_in_coreg_domain(&coreg1,&t1.lifelines_ids(),&t2.lifelines_ids()){
                    quadruples.push((t1, t1_prime.clone(), t2, t2_prime.clone()));
                }
            }
        }
    }

    quadruples
}


#[derive(Clone, PartialEq, Debug, Eq,Hash,Ord,PartialOrd)]
pub enum CoregType{
    Coreg(Vec<usize>),
    Par
}
impl CoregType{

    pub fn to_operator(&self)->Operator{
        match self{
            CoregType::Par=>{
                Operator::Par
            },
            CoregType::Coreg(ids) => {
                if ids.len() == 0{
                    return Operator::Seq;
                }
                else{
                    return Operator::Coreg(ids.clone());
                }
            }
        }
    }
    pub fn to_function_signature(&self)->FunctionSignature{
        match self{
            CoregType::Par=>{
                FunctionSignature::new(self.to_string(),2,Axioms::acu())
            },
            CoregType::Coreg(ids) => {

                FunctionSignature::new(self.to_string(),2,vec![Axioms::A,Axioms::U])
            }
        }

    }
    pub fn parse_from_string(s: &str) -> Option<CoregType> {
        if s == "par" {
            Some(CoregType::Par)
        } else if s == "seq" {
            Some(CoregType::Coreg(vec![]))
        } else if s.starts_with("coreg[") && s.ends_with(']') {
            let content = &s[6..s.len() - 1]; // remove "coreg[" and "]"

            let ids = content
                .split(',')
                .map(|x| x.parse::<usize>().expect("Invalid usize in coreg"))
                .collect();

            Some(CoregType::Coreg(ids))
        } else {
            println!("Invalid CoregType string: {}", s);
            None
        }
    }

    pub fn to_string(&self) -> String{
        match self{
            CoregType::Par => "par".to_string(),
            CoregType::Coreg(ids) => {
                if ids.len() == 0{
                    return "seq".to_string();
                }

                let mut s = "coreg".to_string();
                s.push_str("[");
                for id in ids{
                    s.push_str(&id.to_string());
                    s.push_str(",");
                }
                s.pop();
                s.push(']');
                s
            }
        }
    }

    pub fn merge_coreg(cr1: &CoregType,cr2: &CoregType,lfs1: &Vec<usize>, lfs2: &Vec<usize>) -> CoregType{
        match (cr1.clone(),cr2.clone()){
            (CoregType::Par,CoregType::Par) => {
                CoregType::Par
            },
            (CoregType::Coreg(ids1),CoregType::Par) => {
                let mut all = lfs1.clone();
                all.extend(lfs2.clone());
                all.sort();
                all.dedup();

                let mut ids = ids1.clone();
                ids.extend(lfs2);
                ids.sort();
                ids.dedup();
                if all==ids && all.len()>0{
                    return CoregType::Par;
                }

                CoregType::Coreg(ids)
            },
            (CoregType::Par,CoregType::Coreg(ids2)) => {
                let mut all = lfs1.clone();
                all.extend(lfs2.clone());
                all.sort();
                all.dedup();

                let mut ids = ids2.clone();
                ids.extend(lfs1);
                ids.sort();
                ids.dedup();

                if all==ids && all.len()>0{
                    return CoregType::Par;
                }

                CoregType::Coreg(ids)
            }
            (CoregType::Coreg(ids1),CoregType::Coreg(ids2)) => {
                let mut all = lfs1.clone();
                all.extend(lfs2.clone());
                all.sort();
                all.dedup();


                let mut ids = ids1.clone();
                ids.extend(ids2);
                ids.sort();
                ids.dedup();

                if all==ids && all.len()>0{
                    return CoregType::Par;
                }

                CoregType::Coreg(ids)
            }
        }
    }

    pub fn all_lfs_in_coreg_domain(cr: &CoregType, v1: &Vec<usize>, v2: &Vec<usize>) -> bool {
        match cr{
            CoregType::Par => true,
            CoregType::Coreg(ids) => {
                //Check that all lifelines in v1 and v2 are included in the coreg domain
                for id in v1.iter().chain(v2.iter()) {
                    if !ids.contains(id) {
                        return false;
                    }
                }
                true
            }
        }
    }

    pub fn can_absorb(&self, other: &CoregType) -> bool {
        // Check is lfs(self)  contains  lfs(other)
        match (self, other) {
            (CoregType::Par, CoregType::Par) => true,
            (CoregType::Coreg(_), CoregType::Par) => false,
            (CoregType::Par, CoregType::Coreg(_)) => true,
            (CoregType::Coreg(ids1), CoregType::Coreg(ids2)) => {
                for id in ids2.iter() {
                    if !ids1.contains(id) {
                        return false;
                    }
                }
                true
            }
        }
    }
}

impl Term{
    pub fn is_head_coreg(&self) -> bool {
        match self{
            Term::Variable(_) => false,
            Term::Function(f) => {
                if f.signature.name == "seq" || f.signature.name.contains("coreg") || f.signature.name == "par"{
                    true
                }
                else{
                    false
                }
            }
        }
    }


    pub fn get_coreg_head(&self)->Option<CoregType>{
        match self{
            Term::Variable(_) => None,
            Term::Function(f) => {
                CoregType::parse_from_string(&f.signature.name)
            }
        }
    }

    /* */
    pub fn assoc_coreg_flatten(&self)->Term{
        match self.clone(){
            Term::Variable(_x) => self.clone(),
            Term::Function(f)=>{
                let mut new_args = Vec::new();
                for arg in f.args{
                    match arg.clone(){
                        Term::Variable(_x) => {new_args.push(arg);},
                        Term::Function(g)=>{
                            if self.is_head_coreg() && arg.is_head_coreg(){
                                if let (Some(cr_f),Some(cr_g)) = (CoregType::parse_from_string(&f.signature.name),CoregType::parse_from_string(&g.signature.name)){
                                    if cr_f.can_absorb(&cr_g){
                                        //First we flatten it
                                        let new_arg = arg.assoc_coreg_flatten();

                                        //Then we take its arguments
                                        if let Term::Function(new_g) = new_arg{
                                            new_args.extend(new_g.args.clone());
                                        }
                                    }else{
                                        new_args.push(arg);
                                    }
                                }
                                else{
                                    panic!("Error: cannot parse coreg type");
                                }

                            }
                            else if g.signature== f.signature{
                                //First we flatten it
                                let new_arg = arg.assoc_coreg_flatten();

                                //Then we take its arguments
                                if let Term::Function(new_g) = new_arg{
                                    new_args.extend(new_g.args.clone());
                                }
                            }
                            else{
                                new_args.push(arg);
                            }
                        }
                    }

                }
                Term::Function(Function::new(&f.signature, &new_args))
            }
        }
    }

}



















