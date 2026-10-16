use crate::anti_unification::generalizer::generalizer::Generalizer;
use crate::interactions::composition::error::CompositionError;
use crate::interactions::composition::guideline::Guideline;
use crate::interactions::syntax::interaction::Interaction;
use crate::terms::function::Axioms;

impl Interaction {
    pub fn compose(i1: &Interaction, i2: &Interaction,axioms: &Vec<Axioms>,with_variant_rule: bool,loggers_output_folder:Option<&str>) -> Result<Interaction, CompositionError> {
        let guideline = Guideline::get_guideline(i1, i2)?;

        let t1 = i1.to_term(&guideline,axioms);
        let t2 = i2.to_term(&guideline,axioms);

        //let mut process = GeneralisationProcess::init_process(&t1, &t2);
        //let scp_generalizers = Generalizer::generalize(&t1,&t2,true,false,None);
        let scp_generalizers = if with_variant_rule {
            Generalizer::generalize(&t1,&t2,true,true,loggers_output_folder,None)
        }
        else{
            Generalizer::generalize(&t1,&t2,true,false,loggers_output_folder,None)
        };
        
        if scp_generalizers.len() == 0 {
            return Err(CompositionError::CompositionFailure);
        }
        else{
            let gen = scp_generalizers[0].clone();

            let res = gen.merge(&guideline)?;

            Ok(res)
        }

    }

    pub fn compose_two_pass(i1: &Interaction, i2: &Interaction,axioms: &Vec<Axioms>,with_variants:bool,anti_unif_timout:Option<usize>)-> Result<Interaction,CompositionError>{
        let guideline = Guideline::get_guideline(i1, i2)?;

        let t1 = i1.to_term(&guideline,axioms);
        let t2 = i2.to_term(&guideline,axioms);

        let sc_preserving = true;
        let loggers_output_folder = None;
        //Fist pass
        let mut scp_generalizers = Generalizer::generalize(&t2,&t1,sc_preserving,with_variants,loggers_output_folder,anti_unif_timout);

        /*
        match scp_generalizers.len(){
            0 =>{
                scp_generalizers = Generalizer::generalize(&t2,&t1,true,true,None);
            },
            _ =>{}
        }

         */


        if scp_generalizers.len() == 0 {
            return Err(CompositionError::CompositionFailure);
        }
        else{
            let gen = scp_generalizers[0].clone();

            let res = gen.merge(&guideline)?;

            Ok(res)
        }
    }
}
