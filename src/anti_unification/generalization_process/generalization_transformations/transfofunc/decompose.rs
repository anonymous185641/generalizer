use crate::anti_unification::configuration::aut::AUT;
use crate::anti_unification::configuration::configuration::Configuration;
use crate::terms::function::Function;
use crate::terms::parsing::parser::Rule;
use crate::terms::substitution::substitution::Substitution;
use crate::terms::substitution::variable::Variable;
use crate::terms::term::Term;



pub fn can_apply_decompose(config : &Configuration) -> bool {
    //let aut = self.active[0].clone();
    let aut = &config.active[0];
    if aut.t1.head() == aut.t2.head()
        && !aut.t1.is_head_function_commutative()
        && !aut.t1.is_head_function_associative()
        && !aut.t1.is_head_function_associative_commutative()
    {
        //If arity mismatch, return false
        if let (Term::Function(f1),Term::Function(f2)) = (aut.t1.clone(), aut.t2.clone()) {
            if f1.args.len() == f2.args.len(){
                return true;
            }
        }
    }
    false
}

pub fn transfo_decompose(config : &Configuration) -> Vec<Configuration> {

    // println!("decompose");

    let mut new_active = config.active.clone();
    let new_store = config.store.clone();
    let mut new_sub = config.sub.clone();


    //Since this is decompose, we remove the concerned AUT
    let aut = new_active.remove(0);

    match (&aut.t1.clone(), &aut.t2.clone()) {
        (Term::Function(f1), Term::Function(f2)) => {
            let mut sub = Substitution::new();
            let mut sub_term_args = Vec::new();

            for (t1, t2) in f1.args.iter().zip(f2.args.iter()) {
                let x = Variable::fresh_variable();

                sub_term_args.push(Term::Variable(x.clone()));

                let new_aut = AUT::new(x.clone(), t1.clone(), t2.clone());

                new_active.insert(0, new_aut);
            }
            sub.insert(&aut.x, &Term::Function(Function::new(&aut.t1.head_symbol_signature(), &sub_term_args)));
            new_sub.push(sub);

            //Configuration::new(new_active, new_store,new_sub,self.x0.clone(),self.update_history("Decompose"))
            let new_conf = Configuration::create_new_config(new_active, new_store, new_sub, config.x0.clone());
            vec![new_conf]
        },
        _ => panic!("Error: decompose only works on terms with same head")
    }
}