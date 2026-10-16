use std::fs;
use itertools::Itertools;
use crate::interactions::composition::error::CompositionError;
use crate::interactions::io::output::quick_drawing::draw_model;
use crate::interactions::syntax::general_context::GeneralContext;
use crate::interactions::syntax::interaction::Interaction;
use crate::interactions::syntax::traces::multitrace::MultiTrace;
use crate::interactions::syntax::traces::sync_trace::{SynchronousTrace, ValuePassing};
use crate::interactions::syntax::traces::trace::Trace;
use crate::interactions::syntax::util::special_fold_int::special_fold_recursive_seq_frags;
use crate::interactions::syntax::util::special_get_int::special_get_recursive_seq_frags;

impl Interaction {

    pub fn is_basic(&self) -> bool {
        match self{
            Interaction::Empty => true,
            Interaction::Vp(_,_) => true,
            Interaction::Action(_) => true,
            Interaction::Seq(box1,box2) => (*box1).is_basic() && (*box2).is_basic(),
            _ => false
        }
    }


    pub fn from_trace_to_basic_int(trace: &Trace) -> Interaction {
        if trace.is_empty(){
            return Interaction::Empty;
        }

        let mut int_vec: Vec<Interaction> = Vec::new();



        for action in trace{
            int_vec.push(Interaction::Action(action.clone()));
        }

        special_fold_recursive_seq_frags(&mut int_vec)
    }


    pub fn basic_int_to_synchronous_trace(gen_ctx: &GeneralContext,interaction: &Interaction, draw_folder: Option<&str>)->Result<SynchronousTrace,String>{
        if interaction.is_basic() && interaction.get_free_actions().is_empty(){
            let ints_vec = special_get_recursive_seq_frags(interaction);

            let mut synch_trace = Vec::new();

            for int in ints_vec{


                match int{
                    Interaction::Vp(a1,a2)=>{
                        let vp = ValuePassing::from_pair(&(a1.clone(),a2.clone())).unwrap();
                        synch_trace.push(vp);
                    },
                    Interaction::Empty => {
                        continue;
                    },
                    _ => {
                        return Err("Error: Non closed basic interaction.".to_string());
                    }
                }
            }

            if let Some(folder) = draw_folder{
                println!("Beginning to writing the synchronized traces into files");
                let sync_traces_dir = format!("{}/Synchronized_trace", &folder);
                let _ = fs::create_dir_all(&sync_traces_dir).unwrap();

                let _ = Interaction::sync_trace_to_file(&synch_trace,&gen_ctx,&format!("{}/trace.txt",sync_traces_dir));

            }

            Ok(synch_trace)
        }
        else{
            Err("Error: Non closed basic interaction.".to_string())
        }
    }

    pub fn sync_trace_to_interaction(sync_trace: &SynchronousTrace) -> Interaction {
        let mut vps = Vec::new();

        for st in sync_trace.iter(){
            vps.push(st.to_interaction());
        }

        special_fold_recursive_seq_frags(&mut vps)
    }

    pub fn multitrace_to_global_basic_int(gen_ctx: &GeneralContext,multi_trace: &MultiTrace,draw_folder: Option<&str>) -> Result<Interaction,CompositionError> {
        let mut basic_locals_int: Vec<Interaction> = Vec::new();

        for trace in multi_trace.iter(){
            basic_locals_int.push(Interaction::from_trace_to_basic_int(trace));
        }




        let mut global_int = basic_locals_int[0].clone();

        for i in 0..basic_locals_int.len()-1{

            global_int = Interaction::tag_and_compose_pairwise(&global_int,&basic_locals_int[i+1],&None)?;
            global_int = global_int.iat_canonize(gen_ctx);
        }

        //Drawing interactions.
        if let Some(folder) = draw_folder{
            let local_folder = format!("{}/local_basic_interactions",folder);
            let _ = fs::create_dir_all(&local_folder).unwrap();
            for (ct,interaction) in basic_locals_int.iter().enumerate(){
                draw_model(gen_ctx,&format!("i_{}",ct+1),&local_folder,interaction);
            }

            let global_folder = format!("{}/global_basic_interaction",folder);
            let _ = fs::create_dir_all(&global_folder).unwrap();
            draw_model(gen_ctx,"global_basic_interaction",&global_folder,&global_int);
        }

        Ok(global_int)
    }


    pub fn multitrace_synchronization(gen_ctx: &GeneralContext,multi_trace: &MultiTrace, draw_folder: Option<&str>) -> Result<SynchronousTrace,String>{

        if let Ok(global_int) = Self::multitrace_to_global_basic_int(gen_ctx,multi_trace,draw_folder){
            Self::basic_int_to_synchronous_trace(gen_ctx,&global_int,draw_folder)
        }
        else{
            Err("Synchronization Failure.".to_string())
        }


    }

}