use crate::anti_unification::configuration::configuration::Configuration;
use crate::anti_unification::generalization_process::generalization_transformations::transfofunc::coreg_rules::decompose_ac_coreg::{ac_coreg_comm_dec_groups, CoregType};
use crate::anti_unification::generalization_process::generalization_transformations::transfofunc::decompose_a::assoc_dec_groups;
use crate::terms::function::{Function, FunctionSignature};
use crate::terms::substitution::substitution::Substitution;
use crate::terms::term::Term;

pub fn can_apply_decompose_a_coreg(config : &Configuration) -> bool {

    let aut = config.active[0].clone();

    if aut.t1.is_head_coreg() && aut.t2.is_head_coreg(){

        if aut.t1.head() == aut.t2.head() && aut.t1.head().eq("par") {//If Par on both sides, apply the Decompose_A transformation instead
            return false;
        }

        return true;
    }

    false
}




pub fn transfo_decompose_a_coreg(config : &Configuration) -> Vec<Configuration> {
    let mut return_configurations = Vec::new();

    let mut active = config.active.clone();

    //Since this is decompose, we remove the concerned AUT
    let aut = active.remove(0);

    match (aut.t1.clone().assoc_coreg_flatten(), aut.t2.clone().assoc_coreg_flatten()) {
        (Term::Function(f1), Term::Function(f2)) => {

            println!("\nDecompose_A_Coreg");
            println!("f1: {}",aut.t1.clone().assoc_coreg_flatten());
            println!("f2: {}",aut.t2.clone().assoc_coreg_flatten());
            for (t1,t1_prime,t2,t2_prime) in coreg_assoc_dec_groups(&f1.signature, &f2.signature, &f1.args, &f2.args) {
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



pub fn coreg_assoc_dec_groups(sig1: &FunctionSignature, sig2: &FunctionSignature,args1: &Vec<Term>, args2: &Vec<Term>) -> Vec<(Term,Term,Term,Term)> {
    let mut quadruples = Vec::new();

    //Left
    for i in 0..args1.len()-1 {
        for j in 0..args2.len()-1 {
            let t1 = Term::assoc_wrap(sig1,&Vec::from(&args1[0..=i]));//.to_associative_form();
            let t1_prime = Term::assoc_wrap(sig2,&Vec::from(&args2[0..=j]));

            let t2 = Term::assoc_wrap(sig1,&Vec::from(&args1[i+1..]));//.to_associative_form();
            let t2_prime = Term::assoc_wrap(sig2,&Vec::from(&args2[j+1..]));//.to_associative_form();

            quadruples.push((t1,t1_prime,t2,t2_prime));
        }

    }

    //Right
    /*
    for i in 0..args2.len()-1 {
        for j in 0..args1.len()-1 {
            let t1 =  Term::assoc_wrap(sig1,&Vec::from(&args1[0..=j]));
            let t1_prime = Term::assoc_wrap(sig2,&Vec::from(&args2[0..=i]));//.to_associative_form();

            let t2 = Term::assoc_wrap(sig1,&Vec::from(&args1[1..]));//.to_associative_form();
            let t2_prime = Term::assoc_wrap(sig2,&Vec::from(&args2[i+1..]));//.to_associative_form();


            quadruples.push((t1,t1_prime,t2,t2_prime));
        }

    }

     */



    quadruples
}


/*
pub fn coreg_assoc_dec_groups(sig1: &FunctionSignature, sig2: &FunctionSignature,args1: &Vec<Term>, args2: &Vec<Term>) -> Vec<(Term,Term,Term,Term)> {
    let mut quadruples = Vec::new();

    //Left
    for i in 0..args1.len()-1 {

            let t1 = Term::assoc_wrap(sig1,&Vec::from(&args1[0..=i]));//.to_associative_form();
            let t1_prime = args2[0].clone();

            let t2 = Term::assoc_wrap(sig1,&Vec::from(&args1[i+1..]));//.to_associative_form();
            let t2_prime = Term::assoc_wrap(sig2,&Vec::from(&args2[1..]));//.to_associative_form();

            quadruples.push((t1,t1_prime,t2,t2_prime));


    }

    //Right
    for i in 0..args2.len()-1 {
        let t1 = args1[0].clone();
        let t1_prime = Term::assoc_wrap(sig2,&Vec::from(&args2[0..=i]));//.to_associative_form();

        let t2 = Term::assoc_wrap(sig1,&Vec::from(&args1[1..]));//.to_associative_form();
        let t2_prime = Term::assoc_wrap(sig2,&Vec::from(&args2[i+1..]));//.to_associative_form();


        quadruples.push((t1,t1_prime,t2,t2_prime));
    }



    quadruples
}
 */



