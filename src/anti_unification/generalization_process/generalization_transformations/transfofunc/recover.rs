use crate::anti_unification::configuration::configuration::Configuration;
use crate::terms::substitution::substitution::Substitution;
use crate::terms::term::Term;



pub fn can_apply_recover(config : &Configuration) -> bool {
    let aut = &config.active[0];

    if aut.t1.head_symbol_signature() != aut.t2.head_symbol_signature()
        && !aut.t1.is_head_function_has_unit()
        && !aut.t2.is_head_function_has_unit(){
        for aut2 in &config.store {
            if aut.t1 == aut2.t1 && aut.t2 == aut2.t2 {
                return true;
            }
        }
    }

    false
}


pub fn transfo_recover(config : &Configuration) -> Vec<Configuration> {
    
    let mut new_active = config.active.clone();
    let new_store = config.store.clone();
    let mut new_sub = config.sub.clone();

    let aut = new_active.remove(0);

    let mut sub = Substitution::new();
    for aut2 in &config.store {
        if aut.t1 == aut2.t1 && aut.t2 == aut2.t2 {
            sub.insert(&aut.x, &Term::Variable(aut2.x.clone()));
            new_sub.push(sub);
            return vec![Configuration::create_new_config(new_active, new_store, new_sub, config.x0.clone())];
        }
    }
    
    unreachable!()
}