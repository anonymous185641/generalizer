use std::{env, fs};
use std::process::Command;
use std::time::Instant;
use imageproc::contrast::match_histogram;
use crate::automata::automaton::automaton::Automaton;
use crate::automata::cfsm::system_cfsm::system_cfsm::SystemCFSM;
use crate::automata::communicating_automata::composition::benchmark::caut_benchmark_entry::{BrzozowskiCompositionEntry, Pn2regexCompositionEntry};
use crate::automata::communicating_automata::composition::caut_composition_error::CommunicatingAutomataCompositionError;
use crate::automata::communicating_automata::composition::communicating_automata_to_regex::automata_to_regex_algos::AutomataToRegexAlgo;
use crate::automata::communicating_automata::system_cautomata::cautomata::SystemCommunicatingAutomata;
use crate::automata::transition_systems::transition_systems_composition::ts_composition_error::TsCompositionError;
use crate::interactions::io::output::quick_drawing::draw_model;
use crate::interactions::syntax::interaction::Interaction;
use crate::petri_nets::petri_net_io::petri_net_parsing::petri_parsing_interface::petri_nets_parse_file;
use crate::regex_inf::regex::regex::Regex;
use crate::utils::misc::min_max_vec;

impl SystemCommunicatingAutomata {
    pub fn composition_system_communicating_automata(&self, regex_algo: &AutomataToRegexAlgo, name: &str, target_folder: &str,with_variants:bool,pairwise_comp_timout: Option<usize>,draw:bool)
        ->Result<(Interaction,Option<BrzozowskiCompositionEntry>,Option<Pn2regexCompositionEntry>), CommunicatingAutomataCompositionError>{
        //Output folders
        let output_folder = if target_folder.is_empty(){
            format!("comm_automata_composition_{}",regex_algo.get_algo_name())
        }
        else{
            format!("{}/comm_automata_composition_{}",target_folder,regex_algo.get_algo_name())
        };
        let _ = fs::remove_dir_all(&output_folder);
        let _ = fs::create_dir_all(&output_folder);

        let input_automata_system = format!("{}/input_automata_system", output_folder);
        let _ = fs::create_dir_all(&input_automata_system);

        let step1_into_local_interaction_folder = format!("{}/step1_into_local_interaction", output_folder);
        let _ = fs::create_dir_all(&step1_into_local_interaction_folder);

        let step2_local_interaction_composition_folder = format!("{}/step2_local_interaction_composition", output_folder);
        let _ = fs::create_dir_all(&step2_local_interaction_composition_folder);

        /////////////////////////////////////////////////////////////////////////////////
        /////////////////////////////////////////////////////////////////////////////////
        //Drawing input automata
        /////////////////////////////////////////////////////////////////////////////////
        /////////////////////////////////////////////////////////////////////////////////

        //We draw the transition systems into .g files
        if draw{
            for (i, cautomaton) in self.automata.iter().enumerate(){
                //ts.to_g_file(&ts_name,&ts_folder);
                cautomaton.automaton.draw(&cautomaton.name.0, &input_automata_system);
            }
        }
       

        
        println!("\n Input automata drawing Done.\n");


        /////////////////////////////////////////////////////////////////////////////////
        /////////////////////////////////////////////////////////////////////////////////
        //Step 1: To local interactions
        /////////////////////////////////////////////////////////////////////////////////
        /////////////////////////////////////////////////////////////////////////////////

        println!("Beginning Step 1: Transformation of communicating automata into local interactions.\n");


        let general_context =  self.extract_general_context()?;

        let regex_derivation_folder = format!("{}/regex_derivation_files", step1_into_local_interaction_folder);
        let _ = fs::create_dir_all(&regex_derivation_folder);

        let total_time_start = Instant::now();

        let (regex_family,petrify_data_option) = self.to_regex_family(regex_algo,&regex_derivation_folder,draw)?;

        let mut local_interactions: Vec<Interaction> = Self::translate_regex_to_interaction_with_gates(&general_context,&regex_family)?;

        let range_local_interaction_size = min_max_vec( &local_interactions.clone().into_iter().map(|x| x.size()).collect::<Vec<usize>>());

        //Drawing local interactions
        for (ct,interaction) in local_interactions.iter().enumerate(){
            //Drawing interaction
           let interaction_name = format!("{}_{}",name,ct);
            draw_model(&general_context, &interaction_name, &step1_into_local_interaction_folder, &interaction);
        }


        println!("\nStep 1 Done. Transformation of automata into local interactions successful.\n");

        /////////////////////////////////////////////////////////////////////////////////
        /////////////////////////////////////////////////////////////////////////////////
        //Step 2: Composition of local interactions
        /////////////////////////////////////////////////////////////////////////////////
        /////////////////////////////////////////////////////////////////////////////////

        println!("Beginning Step 2: Composition of local interactions.\n");
        let intermediary_compositions_folder = format!("{}/intermediary_compositions", step2_local_interaction_composition_folder);
        let _ = fs::create_dir_all(& intermediary_compositions_folder);


      //  let mut composition_time_start = Instant::now();
        match Interaction::nary_compose(&local_interactions,&general_context,Some(&intermediary_compositions_folder),with_variants,pairwise_comp_timout){
            Some(global_interaction)=>{
                println!("Global Composition of Interaction Models Successful.");
               // let composition_time = composition_time_start.elapsed();

                let total_time = total_time_start.elapsed();

                let size_global_interaction = global_interaction.size();

                draw_model(&general_context, name, &step2_local_interaction_composition_folder, &global_interaction);


                println!("\nStep 2 Done. Composition of the system of communicating automata successful.\n");
                match regex_algo{
                    AutomataToRegexAlgo::Brzozowski =>{
                        let brzozowski_entry = BrzozowskiCompositionEntry::new(range_local_interaction_size,Some(total_time),Some(size_global_interaction));
                        Ok((global_interaction,Some(brzozowski_entry),None))
                    },
                    AutomataToRegexAlgo::ViaPetriNets=>{
                        let pn2regex_entry = Pn2regexCompositionEntry::new(petrify_data_option,Some(range_local_interaction_size),Some(total_time),Some(size_global_interaction));
                        Ok((global_interaction,None,Some(pn2regex_entry)))
                    },
                    _ =>{
                        Ok((global_interaction,None,None))
                    }

                }

            },
            None=>{
                println!("Global Composition of Interaction Models Failed");
                Err(CommunicatingAutomataCompositionError::CompositionOfLocalInteractionsError(Some(range_local_interaction_size.clone())))
            }
        }

    }
    
    
    
}























