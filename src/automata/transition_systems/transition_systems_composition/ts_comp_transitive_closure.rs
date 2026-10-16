use std::{env, fs};
use std::process::Command;
use crate::automata::automaton::automaton::Automaton;
use crate::automata::transition_systems::transition_systems_composition::ts_composition_error::TsCompositionError;
use crate::interactions::io::output::quick_drawing::draw_model;
use crate::interactions::syntax::interaction::Interaction;
use crate::petri_nets::petri_net_io::petri_net_parsing::petri_parsing_interface::petri_nets_parse_file;
use crate::regex_inf::regex::regex::Regex;



impl Automaton{
    pub fn compose_system_of_transition_systems_transitive_closure(system:&Vec<Automaton>, name: &str, target_folder: &str) ->Result<Interaction,TsCompositionError>{
        //Output folders
        let output_folder = if target_folder.is_empty(){
            "cfsm_system_composition_output_without_petri_nets".to_string()
        }
        else{
            format!("{}/cfsm_system_composition_output_without_petri_nets",target_folder)
        };
        let _ = fs::remove_dir_all(&output_folder);
        let _ = fs::create_dir_all(&output_folder);

        let step1_preprocessing = format!("{}/step1_preprocessing",output_folder);
        let _ = fs::create_dir_all(&step1_preprocessing);

        let step2_into_local_interaction_folder = format!("{}/step2_into_local_interaction",output_folder);
        let _ = fs::create_dir_all(&step2_into_local_interaction_folder);

        let step3_local_interaction_composition_folder = format!("{}/step3_local_interaction_composition",output_folder);
        let _ = fs::create_dir_all(&step3_local_interaction_composition_folder);

        /////////////////////////////////////////////////////////////////////////////////
        /////////////////////////////////////////////////////////////////////////////////
        //Step 1: Preprocessing
        /////////////////////////////////////////////////////////////////////////////////
        /////////////////////////////////////////////////////////////////////////////////

        let ts_folder = format!("{}/transition_systems",&step1_preprocessing);
        let _ = fs::create_dir_all(&ts_folder);


        //We draw the transition systems into .g files
        for (i,ts) in system.iter().enumerate(){
            let ts_name = format!("{}_{}",name,i);
            ts.to_g_file(&ts_name,&ts_folder);
            ts.draw(&ts_name,&ts_folder);
        }


        println!("\nStep 1 Done. Preprocessesing successful.\n");


        /////////////////////////////////////////////////////////////////////////////////
        /////////////////////////////////////////////////////////////////////////////////
        //Step 2: To local interactions
        /////////////////////////////////////////////////////////////////////////////////
        /////////////////////////////////////////////////////////////////////////////////
        println!("Beginning Step 2: Transformation of transition systems into local interactions.\n");

        let general_context = if let Ok(gen_ctx) = Self::extract_general_context(system){
            gen_ctx
        }
        else{
            return Err(TsCompositionError::GeneralContextExtractionError);
        };
        let mut local_interactions: Vec<Interaction> = Vec::new();

        for (i,sys) in system.iter().enumerate(){
            let ts_name = format!("{}_{}",name,i);
            let step2_folder = format!("{}/{}", step2_into_local_interaction_folder, ts_name);
            let _ = fs::create_dir_all(&step2_folder);


            let regex = sys.transitive_closure_old();
            println!("REGEX  {:?}",regex);
            let interaction = if let Ok(int) = Interaction::regex_to_interaction_generic(&regex,
                                                                                         &general_context,
                                                                                         Self::parse_action_from_string)
            {
                int
            }else{
                return Err(TsCompositionError::RegexConversionIntoInteractionError);
            };
            println!("Regex successfully transformed into interaction");

            local_interactions.push(interaction.clone());


            //Drawing interaction
            let interaction_folder = format!("{}/{}_interaction", step2_folder, ts_name);
            let _ = fs::create_dir_all(&interaction_folder);
            draw_model(&general_context,&ts_name,&interaction_folder,&interaction);
        }




        println!("\nStep 2 Done. Transformation of Petri nets into local interactions successful.\n");

        /////////////////////////////////////////////////////////////////////////////////
        /////////////////////////////////////////////////////////////////////////////////
        //Step 3: Composition of local interactions
        /////////////////////////////////////////////////////////////////////////////////
        /////////////////////////////////////////////////////////////////////////////////

        println!("Beginning Step 3: Composition of local interactions.\n");
        let intermediary_compositions_folder = format!("{}/intermediary_compositions",step3_local_interaction_composition_folder);
        let _ = fs::create_dir_all(& intermediary_compositions_folder);



        match Interaction::nary_compose(&local_interactions,&general_context,Some(&intermediary_compositions_folder),true,None){
            Some(global_interaction)=>{
                println!("Global Composition of Interaction Models Successful.");

                draw_model(&general_context,name,&step3_local_interaction_composition_folder,&global_interaction);


                println!("\nStep 3 Done. Composition of CFSMs successful.\n");
                Ok(global_interaction)
            },
            None=>{
                println!("Global Composition of Interaction Models Failed");
                Err(TsCompositionError::CompositionOfLocalInteractionsError)
            }
        }

    }
}























