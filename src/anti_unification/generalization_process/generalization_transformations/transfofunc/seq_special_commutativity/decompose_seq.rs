






use crate::anti_unification::configuration::configuration::Configuration;
use crate::anti_unification::error::ConfigurationError;
use crate::terms::substitution::substitution::Substitution;
use crate::terms::function::{Function, FunctionSignature};
use crate::terms::term::Term;
use crate::utils::combinatorics::Combinatorics;
use crate::utils::misc::have_common_element;

pub fn can_apply_decompose_seq(config : &Configuration) -> bool {
    let aut = config.active[0].clone();

    ///////////////////////////////////////////////
    ///////////////////////////////////////////////
    if aut.seq_special_commutativity_applicable() {
        return true;
    }
    ///////////////////////////////////////////////
    ///////////////////////////////////////////////
    false
}



pub fn transfo_decompose_seq(config : &Configuration) -> Vec<Configuration> {


    let mut return_configurations = Vec::new();

    let mut active = config.active.clone();

    //Since this is decompose, we remove the concerned AUT
    let aut = active.remove(0);

    match (aut.t1.clone().assoc_flatten(), aut.t2.clone().assoc_flatten()) {
        (Term::Function(f1), Term::Function(f2)) => {

            for (t1,t1_prime,t2,t2_prime) in seq_comm_dec_groups(&f1.signature,&f1.args, &f2.args) {
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

                sub.insert(&aut.x, &Term::Function(Function::new(&aut.t1.head_symbol_signature(), &sub_term_args)));
                new_sub.push(sub);

                //let conf = Configuration::new(new_active, new_store,new_sub,config.x0.clone(),config.update_history("Decompose_AC"));

                let conf = Configuration::create_new_config(new_active, new_store, new_sub, config.x0.clone());
                return_configurations.push(conf);
            }

            return return_configurations;
        },
        _ => panic!("Error: decompose_seq only works on terms with same head")
    }
}


pub fn seq_comm_dec_groups(sig: &FunctionSignature,args1: &Vec<Term>, args2: &Vec<Term>) -> Vec<(Term,Term,Term,Term)> {
    let mut quadruples = Vec::new();

    //Left
    for i in 0..args1.len()-1 {
        let t1 = Term::assoc_wrap(sig,&Vec::from(&args1[0..=i]));
        let t2 = Term::assoc_wrap(sig,&Vec::from(&args1[i+1..]));

        let n = args2.len();
        for subset_size in 1..=n - 1 {
            let indices: Vec<usize> = (0..n).collect();
            let combos = Combinatorics::combinations(&indices, subset_size);

            for combo in combos {
                let (vec3, vec4) = Combinatorics::split_vector(args2, &combo);
                let t1_prime = Term::assoc_wrap(sig, &vec3);
                let t2_prime = Term::assoc_wrap(sig, &vec4);

                if !have_common_element(&t1_prime.lifelines_ids(),&t2_prime.lifelines_ids()){
                    quadruples.push((t1.clone(), t1_prime, t2.clone(), t2_prime));
                }

            }
        }
    }

    //Right
    for i in 0..args2.len()-1 {
        let t1_prime = Term::assoc_wrap(sig,&Vec::from(&args2[0..=i]));
        let t2_prime = Term::assoc_wrap(sig,&Vec::from(&args2[i+1..]));

        let n = args1.len();
        for subset_size in 1..=n - 1 {
            let indices: Vec<usize> = (0..n).collect();
            let combos = Combinatorics::combinations(&indices, subset_size);

            for combo in combos {
                let (vec1, vec2) = Combinatorics::split_vector(args1, &combo);

                let t1 = Term::assoc_wrap(sig, &vec1);//.to_associative_form();
                let t2 = Term::assoc_wrap(sig, &vec2);//.to_associative_form();

                if !have_common_element(&t1.lifelines_ids(),&t2.lifelines_ids()){
                    quadruples.push((t1, t1_prime.clone(), t2, t2_prime.clone()));
                }
            }
        }
    }



    quadruples
}




