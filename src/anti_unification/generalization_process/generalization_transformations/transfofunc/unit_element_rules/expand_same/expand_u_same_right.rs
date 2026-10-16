

use crate::anti_unification::configuration::aut::AUT;
use crate::anti_unification::configuration::configuration::Configuration;
use crate::terms::function::Function;
use crate::terms::term::Term;


#[allow(dead_code)]
pub fn can_apply_expand_u_same_right(config : &Configuration) -> bool {
    let aut = config.active[0].clone();

    if aut.t1.head_symbol_signature() == aut.t2.head_symbol_signature()
        && aut.t1.is_head_function_has_unit()
    {
        /////////////////////////
        //TEST
        return true; //for now
        /////////////////////////
        /*
        match (aut.t1.clone(),aut.t2.clone()){
            (Term::Function(f1),Term::Function(f2))=>{
                if f1.args[0] != aut.t1.head_symbol_signature().get_unit()
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





pub fn expand_u_same_right(config : &Configuration) -> Vec<Configuration>  {

    // println!("Expand same");

    let mut new_active = config.active.clone();

    let aut = new_active.remove(0);


    let u_f = aut.t1.head_symbol_signature().get_unit();

    match (aut.t1.clone(), aut.t2.clone()) {
        (Term::Function(_f1), Term::Function(_f2)) => {


            ////////////////////////////////////////////////////////////////////////////
            ////////////////////////////////////////////////////////////////////////////
            ////////////////////////////////////////////////////////////////////////////
            //Expand_Same_Right_1
            //Same as expand_u_right
            let mut new_active3 = new_active.clone();
            let new_store3 = config.store.clone();
            let new_sub3 = config.sub.clone();


            let t_right_3 = if aut.t1.is_head_function_associative(){
                let mut new_args = vec![u_f.clone()];
                new_args.extend(_f2.args.clone());
                Term::Function(Function::new(&aut.t1.head_symbol_signature(), &new_args))
            }
            else{
                Term::Function(Function::new(&aut.t1.head_symbol_signature(), &vec![u_f.clone(), aut.t2.clone()]))
            };
            let new_aut3 = AUT::new(aut.x.clone(), aut.t1.clone(),t_right_3.clone());

            new_active3.insert(0,new_aut3);

            let conf3 = Configuration::create_new_config(new_active3, new_store3, new_sub3, config.x0.clone());



            ////////////////////////////////////////////////////////////////////////////
            ////////////////////////////////////////////////////////////////////////////
            ////////////////////////////////////////////////////////////////////////////
            //Expand_Same_Right_2
            //if config.can_apply_expand_same_right_2(){
            //Same as expand_u_right
            let mut new_active4 = new_active.clone();
            let new_store4 = config.store.clone();
            let new_sub4 = config.sub.clone();

            let t_left_4 = if aut.t1.is_head_function_associative(){
                let mut new_args = _f2.args.clone();
                new_args.extend(vec![u_f]);
                Term::Function(Function::new(&aut.t1.head_symbol_signature(), &new_args))
            }
            else{
                Term::Function(Function::new(&aut.t1.head_symbol_signature(), &vec![aut.t2.clone(), u_f.clone()]))
            };
            let new_aut4 = AUT::new(aut.x.clone(), aut.t1.clone(),t_left_4.clone());

            new_active4.insert(0,new_aut4);

            let conf4 = Configuration::create_new_config(new_active4, new_store4, new_sub4, config.x0.clone());


            //return

            vec![conf3, conf4]
        },
        _ => vec![]
    }
}

