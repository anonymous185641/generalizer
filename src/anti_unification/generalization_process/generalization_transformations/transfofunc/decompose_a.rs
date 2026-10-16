use crate::anti_unification::configuration::configuration::Configuration;
use crate::terms::function::{Function, FunctionSignature};
use crate::terms::substitution::substitution::Substitution;
use crate::terms::term::Term;


pub fn can_apply_decompose_a(config : &Configuration) -> bool {
    let aut = &config.active[0];

    if aut.t1.head_symbol_signature() == aut.t2.head_symbol_signature()
        && aut.t1.is_head_function_associative()
        && !aut.t2.is_head_function_commutative()
    {
        return true;
    }
    false
}



pub fn transfo_decompose_a(config : &Configuration) -> Vec<Configuration> {

    let mut return_configurations = Vec::new();

    let mut active = config.active.clone();

    //Since this is decompose, we remove the concerned AUT
    let aut = active.remove(0);

    //match (aut.t1.clone().assoc_flatten(), aut.t2.clone().assoc_flatten()) {
    match (aut.t1.clone(), aut.t2.clone()) {
        (Term::Function(f1), Term::Function(f2)) => {
            for (t1, t1_prime, t2, t2_prime) in assoc_dec_groups(&f1.signature, &f1.args, &f2.args) {
                let mut new_active = active.clone();
                let new_store = config.store.clone();
                let mut new_sub = config.sub.clone();


                let (x1, x2, aut1, aut2) = config.with_fresh_aut_pair(
                    t1, t1_prime, t2, t2_prime
                );

                new_active.insert(0, aut1.clone());
                new_active.insert(0, aut2.clone());

                let mut sub = Substitution::new();
                let sub_term_args = vec![Term::Variable(x1), Term::Variable(x2)];

                sub.insert(&aut.x, &Term::Function(Function::new(&aut.t1.head_symbol_signature(), &sub_term_args)));
                new_sub.push(sub);

                //let conf = Configuration::new(new_active, new_store,new_sub,config.x0.clone(),config.update_history("Decompose_A"));
                let conf = Configuration::create_new_config(new_active, new_store, new_sub, config.x0.clone());
                return_configurations.push(conf);
            }

            return_configurations
        },
        _ => panic!("Error: decompose_a only works on terms with same head")
    }
}


pub fn assoc_dec_groups(sig: &FunctionSignature,args1: &Vec<Term>, args2: &Vec<Term>) -> Vec<(Term,Term,Term,Term)> {
    let mut quadruples = Vec::new();

    //Left
    for i in 0..args1.len()-1 {
        let t1 = Term::assoc_wrap(sig,&Vec::from(&args1[0..=i]));//.to_associative_form();
        let t1_prime = args2[0].clone();

        let t2 = Term::assoc_wrap(sig,&Vec::from(&args1[i+1..]));//.to_associative_form();
        let t2_prime = Term::assoc_wrap(sig,&Vec::from(&args2[1..]));//.to_associative_form();

        quadruples.push((t1,t1_prime,t2,t2_prime));
    }

    //Right
    for i in 0..args2.len()-1 {
        let t1 = args1[0].clone();
        let t1_prime = Term::assoc_wrap(sig,&Vec::from(&args2[0..=i]));//.to_associative_form();

        let t2 = Term::assoc_wrap(sig,&Vec::from(&args1[1..]));//.to_associative_form();
        let t2_prime = Term::assoc_wrap(sig,&Vec::from(&args2[i+1..]));//.to_associative_form();


        quadruples.push((t1,t1_prime,t2,t2_prime));
    }



    quadruples
}