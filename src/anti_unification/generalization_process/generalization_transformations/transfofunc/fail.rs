use crate::anti_unification::configuration::configuration::Configuration;


pub fn can_apply_fail(config : &Configuration) -> bool{

    for aut in config.active.iter(){

        if aut.t1.get_special_constants() != aut.t2.get_special_constants()
        {
           // println!("fail");
            return true;
        }
    }

    false
}


pub fn transfo_fail(config : &Configuration) -> Vec<Configuration> {
    vec![]
}