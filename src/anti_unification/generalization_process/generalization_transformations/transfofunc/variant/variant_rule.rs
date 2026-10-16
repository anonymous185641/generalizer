use crate::anti_unification::configuration::aut::AUT;
use crate::anti_unification::configuration::configuration::Configuration;
use crate::anti_unification::generalization_process::generalization_transformations::transfofunc::variant::variant::Variant;
use crate::matching::brute_force::matching_ac::bf_match_ac;
use crate::terms::function::Function;
use crate::terms::substitution::substitution::Substitution;
use crate::terms::substitution::variable::Variable;
use crate::terms::term::Term;

pub fn can_apply_variant(config: &Configuration) ->bool{
    let aut = config.active[0].clone();

    //////////////////////////////////////////////////////////////
    /*
    if aut.t1.get_special_constants().len()==0 || aut.t2.get_special_constants().len()==0{
        return false;
    }
     */

    //////////////////////////////////////////////////////////////
    let signature  = Term::special_get_interaction_signature(&vec![aut.t1.clone(),aut.t2.clone()]);
    
    for function in signature.iter(){

        match function.arity{
            0 => {
                continue
            },
            1 => {
                //println!("Here arity1 ??");
                let x1 = Variable::fresh_variable();

                let x1_term = Term::Variable(x1.clone());

                let f_term = Term::Function(Function::new(&function,&vec![x1_term]));

                let variants_f_term = Variant::get_variants(&f_term,&signature,&vec![x1.clone()],true);

                for variant1 in variants_f_term.iter(){
                    for variant2 in variants_f_term.iter(){
                        if let (Some(rho1),Some(rho2)) = (bf_match_ac(&variant1.term,&aut.t1),bf_match_ac(&variant2.term,&aut.t2)){
                            return true;
                        }
                    }

                }

            },
            2 =>{
                //println!("Here arity2 ??");
                let x1 = Variable::fresh_variable();
                let x2 = Variable::fresh_variable();

                let x1_term = Term::Variable(x1.clone());
                let x2_term = Term::Variable(x2.clone());

                let f_term = Term::Function(Function::new(&function,&vec![x1_term, x2_term]));

                let variants_f_term = Variant::get_variants(&f_term,&signature,&vec![x1.clone(),x2.clone()],false);


                for variant1 in variants_f_term.iter(){
                    for variant2 in variants_f_term.iter(){
                        if let (Some(rho1),Some(rho2)) = (bf_match_ac(&variant1.term,&aut.t1),bf_match_ac(&variant2.term,&aut.t2)){
                            return true;
                        }
                    }

                }

            },
            _ =>{
                panic!("Arity Not supported yet");
            }
        }
    }
    
    false
}




pub fn transfo_variant(config: &Configuration) ->Vec<Configuration>{

    let aut = config.active[0].clone();

    let signature  = Term::special_get_interaction_signature(&vec![aut.t1.clone(),aut.t2.clone()]);
    //println!("signature: {:?}",signature);
    let mut output_configs = Vec::new();

    for function in signature.iter(){

        match function.arity{
            0 => {
                continue
            },
            1 => {
                let x1 = Variable::fresh_variable();

                let x1_term = Term::Variable(x1.clone());

                let f_term = Term::Function(Function::new(&function,&vec![x1_term.clone()]));
               // println!("IN");
                let variants_f_term = Variant::get_variants(&f_term,&signature,&vec![x1.clone()],true);
                //println!("OUT");

                //println!("Variants all arity 1 : {:?}",variants_f_term);
                for variant1 in variants_f_term.iter(){
                    for variant2 in variants_f_term.iter(){
                        //println!("Here 0000 1111 ??");
                        if let (Some(rho1),Some(rho2)) = (bf_match_ac(&variant1.term,&aut.t1),bf_match_ac(&variant2.term,&aut.t2)){

                            //println!("LOOP: variant1: {} variant2: {}",variant1.term,variant2.term);
                            let sigma1 = variant1.substitution.clone();
                            let sigma2 = variant2.substitution.clone();

                            let mut  new_active = config.active.clone();
                            let new_store = config.store.clone();
                            let mut new_sub = config.sub.clone();

                            //We remove the first AUT
                            let _ = new_active.remove(0);

                            //We save the function in the substitution
                            let mut sub = Substitution::new();
                            sub.insert(&aut.x, &Term::Function(Function::new(&function,&vec![x1_term.clone()])));
                            new_sub.push(sub);

                            //New AUTs
                            let new_term1_left = x1_term.apply_substitution(&sigma1).apply_substitution(&rho1);
                            let new_term1_right = x1_term.apply_substitution(&sigma2).apply_substitution(&rho2);


                            let new_aut1 = AUT::new(x1.clone(),new_term1_left,new_term1_right);

                            new_active.insert(0,new_aut1.clone());

                            let new_conf = Configuration::create_new_config(new_active, new_store, new_sub, config.x0.clone());


                            ///////////////////////////////////////////////////
                            //////////////////////////////////////////////////
                            /**/
                            if (new_aut1.t1 == aut.t1 && new_aut1.t2 == aut.t2){
                                continue;
                            }




                            //////////////////////////////////////////////////
                            /////////////////////////////////////////////////
                            //println!("new_conf: {}",new_conf);
                            output_configs.push(new_conf);


                        }
                    }

                }
            },
            2 =>{
                let x1 = Variable::fresh_variable();
                let x2 = Variable::fresh_variable();

                let x1_term = Term::Variable(x1.clone());
                let x2_term = Term::Variable(x2.clone());

                let f_term = Term::Function(Function::new(&function,&vec![x1_term.clone(), x2_term.clone()]));

                let variants_f_term = Variant::get_variants(&f_term,&signature,&vec![x1.clone(),x2.clone()],false);


                for variant1 in variants_f_term.iter(){
                    for variant2 in variants_f_term.iter(){
                        //println!("Here 0000 ??");
                        if let (Some(rho1),Some(rho2)) = (bf_match_ac(&variant1.term,&aut.t1),bf_match_ac(&variant2.term,&aut.t2)){
                           // println!("variant1: {} variant2: {}",variant1.term,variant2.term);
                            let sigma1 = variant1.substitution.clone();
                            let sigma2 = variant2.substitution.clone();


                            let mut  new_active = config.active.clone();
                            let new_store = config.store.clone();
                            let mut new_sub = config.sub.clone();

                            //We remove the first AUT
                            let _ = new_active.remove(0);

                            //We save the function in the substitution
                            let mut sub = Substitution::new();
                            sub.insert(&aut.x, &Term::Function(Function::new(&function,&vec![x1_term.clone(), x2_term.clone()])));
                            new_sub.push(sub);

                            //New AUTs

                            let new_term1_left = x1_term.apply_substitution(&sigma1).apply_substitution(&rho1);
                            let new_term1_right = x1_term.apply_substitution(&sigma2).apply_substitution(&rho2);


                            let new_term2_left = x2_term.apply_substitution(&sigma1).apply_substitution(&rho1);
                            let new_term2_right = x2_term.apply_substitution(&sigma2).apply_substitution(&rho2);


                            let new_aut1 = AUT::new(x1.clone(),new_term1_left,new_term1_right);
                            let new_aut2 = AUT::new(x2.clone(),new_term2_left,new_term2_right);

                            new_active.insert(0,new_aut2.clone());
                            new_active.insert(0,new_aut1.clone());

                            let new_conf = Configuration::create_new_config(new_active, new_store, new_sub, config.x0.clone());


                            ///////////////////////////////////////////////////
                            //////////////////////////////////////////////////
                            /**/
                            if (new_aut1.t1 == aut.t1 && new_aut1.t2 == aut.t2)|| (new_aut2.t1 == aut.t1 && new_aut2.t2 == aut.t2){
                                continue;
                            }

                            //println!("Here 11111 ??");
                            //////////////////////////////////////////////////
                            /////////////////////////////////////////////////
                            //println!("new_conf: {}",new_conf);
                            output_configs.push(new_conf);


                        }
                    }

                }
            },
            _ =>{
                panic!("Arity Not supported yet");
            }
        }




    }



    output_configs
}
















