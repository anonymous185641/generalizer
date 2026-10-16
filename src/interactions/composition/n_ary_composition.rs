use crate::interactions::io::output::quick_drawing::{draw_model, draw_model_without_hif};
use crate::interactions::syntax::general_context::GeneralContext;
use crate::interactions::syntax::interaction::Interaction;
use crate::terms::function::Axioms;

impl Interaction{
    pub fn nary_compose(interactions: &Vec<Interaction>,gen_ctx: &GeneralContext,draw_folder:Option<&str>,with_variants:bool,pairwise_comp_timout:Option<usize>)-> Option<Interaction> {
        if interactions.len() < 2 {
            return None;
        }

        let mut result = interactions[0].clone();

        for i in 1..interactions.len(){

            //result = match Interaction::compose(&result,&interactions[i],&Axioms::acu(),false,None){
            result = match Interaction::compose_two_pass(&result,&interactions[i],&Axioms::acu(),with_variants,pairwise_comp_timout){
                Ok(res) => {
                            //println!("{:?}",res);
                            if let Some(folder) = draw_folder{
                                draw_model_without_hif(gen_ctx,&format!("result_{}",i),folder,&res);
                            }
    
                            res.iat_canonize(gen_ctx)
                        },
                        Err(_) => return None
                    };

        }
        Some(result)
    }
}














