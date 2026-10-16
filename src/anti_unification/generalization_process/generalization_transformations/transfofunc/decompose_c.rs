use crate::anti_unification::configuration::aut::AUT;
use crate::anti_unification::configuration::configuration::Configuration;
use crate::terms::function::Function;
use crate::terms::substitution::substitution::Substitution;
use crate::terms::substitution::variable::Variable;
use crate::terms::term::Term;



pub fn can_apply_decompose_c(config : &Configuration) -> bool {
    let aut =&config.active[0];
    
    if aut.t1.head_symbol_signature() == aut.t2.head_symbol_signature()
        && aut.t1.is_head_function_commutative()
        && !aut.t2.is_head_function_associative()
    {
        return true;
    }
    false
}



pub fn transfo_decompose_c(config : &Configuration) -> Vec<Configuration> {

    let mut new_active = config.active.clone();

    //Since this is decompose, we remove the concerned AUT
    let aut = new_active.remove(0);

    match (aut.t1.clone(), aut.t2.clone()) {
        (Term::Function(f1), Term::Function(f2)) => {


            //Return config 1
            let mut new_active1 = new_active.clone();
            let new_store1 = config.store.clone();
            let mut new_sub1 = config.sub.clone();

            let mut sub1 = Substitution::new();
            let mut sub_term_args1 = Vec::new();

            for (t1, t2) in f1.args.iter().zip(f2.args.iter()) {
                let x1 = Variable::fresh_variable();

                sub_term_args1.push(Term::Variable(x1.clone()));

                let new_aut1 = AUT::new(x1.clone(), t1.clone(), t2.clone());

                new_active1.insert(0,new_aut1);
            }
            sub1.insert(&aut.x, &Term::Function(Function::new(&aut.t1.head_symbol_signature(), &sub_term_args1)));
            new_sub1.push(sub1);

            //let conf1 = Configuration::new(new_active1, new_store1,new_sub1,config.x0.clone(),config.update_history("Decompose_C"));
            let conf1 = Configuration::create_new_config(new_active1, new_store1, new_sub1, config.x0.clone());
            //Return config 2

            let mut new_active2 = new_active.clone();
            let new_store2 = config.store.clone();
            let mut new_sub2 = config.sub.clone();

            let mut sub2 = Substitution::new();
            let mut sub_term_args2 = Vec::new();

            //let mut f2_args_rev = f2.args.clone();
            //f2_args_rev.reverse();

            for (t1, t2) in f1.args.iter().zip(f2.args.iter().rev()) {
                let x2 = Variable::fresh_variable();

                sub_term_args2.push(Term::Variable(x2.clone()));

                let new_aut2 = AUT::new(x2.clone(), t1.clone(), t2.clone());

                new_active2.insert(0,new_aut2);
            }
            sub2.insert(&aut.x, &Term::Function(Function::new(&aut.t1.head_symbol_signature(), &sub_term_args2)));
            new_sub2.push(sub2);

            //let conf2 = Configuration::new(new_active2, new_store2,new_sub2,config.x0.clone(),config.update_history("Decompose_C"));
            let conf2 = Configuration::create_new_config(new_active2, new_store2, new_sub2, config.x0.clone());
            //return
            vec![conf1, conf2]
        },
        _ => panic!("Decomposition only applies for functions with same head")
    }
}