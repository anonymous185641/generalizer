use crate::anti_unification::configuration::configuration::Configuration;



pub fn can_apply_solve(config : &Configuration) -> bool {
    let aut = &config.active[0];

    if aut.t1.head_symbol_signature() != aut.t2.head_symbol_signature()
        && !aut.t1.is_head_function_has_unit()
        && !aut.t2.is_head_function_has_unit(){
        for aut2 in &config.store {
            if aut.t1 == aut2.t1 && aut.t2 == aut2.t2 {
                return false;
            }
        }
        return true;
    }
    false
}



pub fn transfo_solve(config : &Configuration) -> Vec<Configuration> {
    let mut new_active = config.active.clone();
    let mut new_store = config.store.clone();
    let new_sub = config.sub.clone();

    let aut = new_active.remove(0);

    new_store.push(aut);

    //Configuration::new(new_active,new_store,new_sub,self.x0.clone(),self.update_history("Solve"))

    vec![Configuration::create_new_config(new_active, new_store, new_sub, config.x0.clone())]
}