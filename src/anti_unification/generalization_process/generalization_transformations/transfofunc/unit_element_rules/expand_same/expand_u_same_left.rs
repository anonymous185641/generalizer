

use crate::anti_unification::configuration::aut::AUT;
use crate::anti_unification::configuration::configuration::Configuration;


use crate::terms::function::Function;
use crate::terms::term::Term;

#[allow(dead_code)]
pub fn can_apply_expand_u_same_left(config : &Configuration) -> bool {
    let aut = config.active[0].clone();

    if aut.t1.head_symbol_signature() == aut.t2.head_symbol_signature()
        && aut.t1.is_head_function_has_unit()
    {
        /////////////////////////
        //TEST
        return true; //For now
        /////////////////////////
        /*
        match (aut.t1.clone(),aut.t2.clone()) {
            (Term::Function(f1),Term::Function(f2))=>{
                if //f1.args[0] != aut.t1.head_symbol_signature().get_unit() &&
                f2.args[0] != aut.t2.head_symbol_signature().get_unit()
                {
                    return true;
                }
            },
            _ => {}
        }

         */
    }
    false


}



pub fn expand_u_same_left(config : &Configuration) -> Vec<Configuration>  {

    // println!("Expand same");

    let mut new_active = config.active.clone();

    let aut = new_active.remove(0);



    let u_f = aut.t1.head_symbol_signature().get_unit();

    match (aut.t1.clone(), aut.t2.clone()) {
        (Term::Function(_f1), Term::Function(_f2)) => {

            ////////////////////////////////////////////////////////////////////////////
            ////////////////////////////////////////////////////////////////////////////
            ////////////////////////////////////////////////////////////////////////////
            //Expand_Same_Left_1


            let mut new_active1 = new_active.clone();
            let new_store1 = config.store.clone();
            let new_sub1 = config.sub.clone();


            let t_left_1 = if aut.t2.is_head_function_associative(){
                let mut new_args = vec![u_f.clone()];
                new_args.extend(_f1.args.clone());
                Term::Function(Function::new(&aut.t2.head_symbol_signature(), &new_args))
            }
            else{
                Term::Function(Function::new(&aut.t2.head_symbol_signature(), &vec![u_f.clone(), aut.t1.clone()]))
            };

            let new_aut1 = AUT::new(aut.x.clone(), t_left_1.clone(), aut.t2.clone());

            new_active1.insert(0,new_aut1);

            let conf1 = Configuration::create_new_config(new_active1, new_store1, new_sub1, config.x0.clone());






            ////////////////////////////////////////////////////////////////////////////
            ////////////////////////////////////////////////////////////////////////////
            ////////////////////////////////////////////////////////////////////////////
            //Expand_Same_Left_2
            let mut new_active2 = new_active.clone();
            let new_store2 = config.store.clone();
            let new_sub2 = config.sub.clone();



            let t_left_2 = if aut.t2.is_head_function_associative(){
                let mut new_args =_f1.args.clone();
                new_args.extend(vec![u_f]);
                Term::Function(Function::new(&aut.t2.head_symbol_signature(), &new_args))
            }
            else{
                Term::Function(Function::new(&aut.t2.head_symbol_signature(), &vec![aut.t1.clone(), u_f.clone()]))
            };

            let new_aut2 = AUT::new(aut.x.clone(), t_left_2.clone(), aut.t2.clone());

            new_active2.insert(0,new_aut2);

            let conf2 = Configuration::create_new_config(new_active2, new_store2, new_sub2, config.x0.clone());



            //return

            vec![conf1, conf2]
        },
        _ => vec![]
    }
}

