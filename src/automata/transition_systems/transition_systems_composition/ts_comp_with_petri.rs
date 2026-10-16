use std::{env, fs};
use std::process::Command;
use itertools::Itertools;
use crate::automata::automaton::automaton::Automaton;
use crate::automata::transition_systems::transition_systems_composition::ts_composition_error::TsCompositionError;
use crate::interactions::io::output::draw_interactions::interface::draw_interaction;
use crate::interactions::io::output::quick_drawing::draw_model;
use crate::interactions::syntax::action::{Action, ActionType};
use crate::interactions::syntax::general_context::GeneralContext;
use crate::interactions::syntax::interaction::Interaction;
use crate::petri_nets::petri_net_io::petri_net_parsing::petri_parsing_interface::petri_nets_parse_file;
use crate::regex_inf::regex::regex::Regex;

impl Automaton{

    pub fn compose_system_of_transition_systems_via_petri_nets(system:&Vec<Automaton>, name: &str, target_folder: &str) ->Result<Interaction,TsCompositionError>{


        //Output folders
        let output_folder = if target_folder.is_empty(){
            "cfsm_system_composition_ouput".to_string()
        }
        else{
            format!("{}/cfsm_system_composition_ouput",target_folder)
        };
        let _ = fs::remove_dir_all(&output_folder);
        let _ = fs::create_dir_all(&output_folder);


        let step1_into_petri_folder = format!("{}/step1_into_petri_nets",output_folder);
        let _ = fs::create_dir_all(&step1_into_petri_folder);

        let step2_into_local_interaction_folder = format!("{}/step2_into_local_interaction",output_folder);
        let _ = fs::create_dir_all(&step2_into_local_interaction_folder);

        let step3_local_interaction_composition_folder = format!("{}/step3_local_interaction_composition",output_folder);
        let _ = fs::create_dir_all(&step3_local_interaction_composition_folder);


        /////////////////////////////////////////////////////////////////////////////////
        /////////////////////////////////////////////////////////////////////////////////
        //Step 1: Transformation into Petri nets (Petrify)
        /////////////////////////////////////////////////////////////////////////////////
        /////////////////////////////////////////////////////////////////////////////////

        let ts_folder = format!("{}/transition_systems",&step1_into_petri_folder);
        let _ = fs::create_dir_all(&ts_folder);

        let petri_folder = format!("{}/petri_nets",&step1_into_petri_folder);
        let _ = fs::create_dir_all(&petri_folder);

        //We draw the transition systems into .g files
        for (i,ts) in system.iter().enumerate(){
                let ts_name = format!("{}_{}",name,i);
                ts.to_g_file(&ts_name,&ts_folder);
                ts.draw(&ts_name,&ts_folder);

            //Petrify call
            let exe_dir = env::current_exe()
                .unwrap()
                .parent()
                .unwrap()
                .to_path_buf();
            println!("{:?}",petri_folder);
            let petrify = exe_dir.join("petrify");
            let status = Command::new(petrify)
                .arg(&format!("{}/{}.g",ts_folder,ts_name))
                .arg("-dead")
                .arg("-o")
                .arg(&format!("{}/{}.out",petri_folder,ts_name))
                .output();

            /*
            match status {
                Ok(_) => {
                    println!("{} successfully transformed into Petri net.",ts_name);
                },
                Err(e) => {
                    println!("{} transformation into Petri net failed.",ts_name);
                    println!("Error: {}",e);
                    return Err(TsCompositionError::TransformationIntoPetriNetsError);
                },
            }

             */

            match status {
                Ok(output) => {
                    if output.status.success() {
                        println!("{} successfully transformed into Petri net.", ts_name);
                    } else {
                        println!(
                            "{} transformation failed with exit code {:?}",
                            ts_name,
                            output.status.code()
                        );

                        println!(
                            "stderr:\n{}",
                            String::from_utf8_lossy(&output.stderr)
                        );

                        println!(
                            "stdout:\n{}",
                            String::from_utf8_lossy(&output.stdout)
                        );
                    }
                }
                Err(e) => {
                    println!("{} transformation into Petri net failed.", ts_name);
                    println!("Error: {}", e);
                }
            }

        }


        println!("\nStep 1 Done. Transformation into Petri nets successful.\n");

        /////////////////////////////////////////////////////////////////////////////////
        /////////////////////////////////////////////////////////////////////////////////
        //Step 2: Transformation of Petri nets into local interactions
        /////////////////////////////////////////////////////////////////////////////////
        /////////////////////////////////////////////////////////////////////////////////
        println!("Beginning Step 2: Transformation of Petri nets into local interactions.\n");

        let general_context = if let Ok(gen_ctx) = Self::extract_general_context(system){
            gen_ctx
        }
        else{
            return Err(TsCompositionError::GeneralContextExtractionError);
        };
        let mut local_interactions: Vec<Interaction> = Vec::new();

        for (i,_) in system.iter().enumerate(){
            let petri_name = format!("{}_{}",name,i);
            let petri_file_path = format!("{}/{}.out",petri_folder,petri_name);
            let step2_petri_folder = format!("{}/{}", step2_into_local_interaction_folder, petri_name);
            let _ = fs::create_dir_all(&step2_petri_folder);

            match petri_nets_parse_file::<Regex<String>>(&petri_file_path){
                Ok(mut pn)=>{
                    println!("Petri net {}.out successfully parsed.",petri_name);

                    //We eliminate duplicated transitions
                    //For example the transition labeled a/1, or a/2... we become simply a
                    pn.eliminate_duplicated_transition_label();

                    //Converting the Petri net into a regex
                    match pn.petri_to_regex(&petri_name, Some(&step2_petri_folder), false,true){
                        Ok(regex)=>{
                            println!("Petri net {}.out successfully transformed into local interactions.",petri_name);

                            let interaction_folder = format!("{}/{}_interaction", step2_petri_folder, petri_name);
                            let _ = fs::create_dir_all(&interaction_folder);

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
                            draw_model(&general_context,&petri_name,&interaction_folder,&interaction);

                            
                            },
                        Err(e)=>{
                            println!("Error during the transformation of the Petri nets into local interactions.");
                            println!("Error: {}",e);

                            return Err(TsCompositionError::TransformationOfPetriNetsIntoLocalInteractionsError(e));
                        }
                    }

                },
                Err(e)=>{
                    println!("Error during the parsing of the Petri nets.");
                    println!("Error: {}",e);
                    return Err(TsCompositionError::PetriNetParsingError);
                }
            }
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

    pub fn extract_general_context(system:&Vec<Automaton>)->Result<GeneralContext,String>{
        let mut  general_context = GeneralContext::new();
        let mut lfs_ids = vec![];
        let mut messages = vec![];


        for automaton in system{
            for label in automaton.graph.edge_weights(){
                if label.eq(&"Epsilon".to_string()){
                    continue;
                }

                let (action_type, rest) = if let Some(rest) = label.strip_prefix("Emission_l") {
                    (ActionType::Emission, rest)
                } else if let Some(rest) = label.strip_prefix("Reception_l") {
                    (ActionType::Reception, rest)
                } else {
                    return Err("Invalid prefix".into());
                };

                let mut parts = rest.splitn(3, '_');

                let lifeline_id = parts
                    .next()
                    .ok_or("Missing lifeline id")?
                    .parse::<usize>()
                    .map_err(|_| "Invalid lifeline id")?;

                let message = parts
                    .next()
                    .ok_or("Missing message")?
                    .to_string();


                lfs_ids.push(lifeline_id);
                messages.push(message);
            }
        }

        for lifeline_id in lfs_ids.iter().unique(){
            general_context.add_lf(format!("l{}",lifeline_id));
        }

        for message in messages.iter().unique(){
            general_context.add_msg(message.clone());
        }

        Ok(general_context)
    }


    pub fn parse_action_from_string(general_context: &GeneralContext,act_str: &String)->Result<Action,String>{
        /*
            The action of the form:
            Emission_l<lifeline_id>_<Message>_<GateId>
            or Reception_l<lf_id>_<Message>_<GateId> 
        */

        let (action_type, rest) = if let Some(rest) = act_str.strip_prefix("Emission_l") {
            (ActionType::Emission, rest)
        } else if let Some(rest) = act_str.strip_prefix("Reception_l") {
            (ActionType::Reception, rest)
        } else {
            return Err("Invalid prefix".into());
        };

        let mut parts = rest.splitn(3, '_');

        let lifeline_id = parts
            .next()
            .ok_or("Missing lifeline id")?
            .parse::<usize>()
            .map_err(|_| "Invalid lifeline id")?;

        let message = parts
            .next()
            .ok_or("Missing message")?
            .to_string();

        let gate_id = parts
            .next()
            .ok_or("Missing gate id")?
            .parse::<usize>()
            .map_err(|_| "Invalid gate id")?;

        let message_id = if let Some(ms_id) = general_context.get_ms_id(&message){
            ms_id
        }
        else{
            return Err("Message not found".into());
        };

        Ok( Action::new_with_id(lifeline_id,message_id,action_type,gate_id))
    }


    pub fn simple_parse_action_from_string(act_str: &String)->Result<String,String>{
        /*
            The action of the form:
            Emission_l<lifeline_id>_<Message>_<GateId>
            or Reception_l<lf_id>_<Message>_<GateId> 
        */

        let (action_type, rest) = if let Some(rest) = act_str.strip_prefix("Emission_l") {
            (ActionType::Emission, rest)
        } else if let Some(rest) = act_str.strip_prefix("Reception_l") {
            (ActionType::Reception, rest)
        } else {
            return Err("Invalid prefix".into());
        };

        let mut parts = rest.splitn(3, '_');

        let lifeline_id = parts
            .next()
            .ok_or("Missing lifeline id")?
            .parse::<usize>()
            .map_err(|_| "Invalid lifeline id")?;

        let message = parts
            .next()
            .ok_or("Missing message")?
            .to_string();

        let gate_id = parts
            .next()
            .ok_or("Missing gate id")?
            .parse::<usize>()
            .map_err(|_| "Invalid gate id")?;
        
        if action_type==ActionType::Emission{
            Ok( format!("l{}!{}[{}]",lifeline_id,message,gate_id))
        }
        else{
             Ok( format!("l{}?{}[{}]",lifeline_id,message,gate_id))
        }
    }
}



