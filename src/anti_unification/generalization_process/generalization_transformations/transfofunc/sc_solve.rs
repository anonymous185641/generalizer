use crate::anti_unification::configuration::configuration::Configuration;
use crate::anti_unification::generalization_process::generalization_transformations::transfofunc::solve::transfo_solve;

pub fn can_apply_sc_solve(config : &Configuration) -> bool {
    let aut = &config.active[0];

    if (aut.t1.head_symbol_signature() != aut.t2.head_symbol_signature())
        && aut.t1.get_special_constants().is_empty()
        && aut.t2.get_special_constants().is_empty()
        && !aut.t1.is_head_function_has_unit()
        && !aut.t2.is_head_function_has_unit()
    {
        for aut2 in &config.store {
            if aut.t1 == aut2.t1 && aut.t2 == aut2.t2 {
                return false;
            }
        }
        return true;
    }

    false
}
pub fn transfo_sc_solve(config : &Configuration) -> Vec<Configuration> {
    transfo_solve(config)
}