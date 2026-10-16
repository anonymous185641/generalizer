use std::fs;
use crate::automata::automaton::automaton::Automaton;
use crate::automata::communicating_automata::composition::caut_composition_error::CommunicatingAutomataCompositionError;
use crate::automata::communicating_automata::composition::communicating_automata_to_regex::automata_to_regex_algos::AutomataToRegexAlgo;
use crate::automata::communicating_automata::product::sync_product::sync_product::SyncProduct;
use crate::automata::communicating_automata::system_cautomata::cautomata::SystemCommunicatingAutomata;
use crate::interactions::io::output::quick_drawing::draw_model;
use crate::interactions::syntax::interaction::Interaction;

impl SystemCommunicatingAutomata{

    pub fn sync_composition_system_communicating_automata(&self,regex_algo: &AutomataToRegexAlgo,name:&str,target_folder:&str)->Result<Interaction, CommunicatingAutomataCompositionError>{

        //Synchronous product of the automata
        //Output folders
        let output_folder = if target_folder.is_empty(){
            format!("Composition_From_Sync_Product/Using_{}",regex_algo.get_algo_name())
        }
        else{
            format!("{}/Using_{}",target_folder,regex_algo.get_algo_name())
        };
        let _ = fs::remove_dir_all(&output_folder);
        let _ = fs::create_dir_all(&output_folder);

        let input_automata_system = format!("{}/input_automata_system", output_folder);
        let _ = fs::create_dir_all(&input_automata_system);

        let step1_synchronous_product = format!("{}/step1_synchronous_product", output_folder);
        let _ = fs::create_dir_all(&step1_synchronous_product);

        let step2_global_interaction_derivation = format!("{}/step2_global_interaction_derivation", output_folder);
        let _ = fs::create_dir_all(&step2_global_interaction_derivation);

        /////////////////////////////////////////////////////////////////////////////////
        /////////////////////////////////////////////////////////////////////////////////
        //Drawing input automata
        /////////////////////////////////////////////////////////////////////////////////
        /////////////////////////////////////////////////////////////////////////////////

        //We draw the transition systems into .g files
        for (i, cautomaton) in self.automata.iter().enumerate(){
            //ts.to_g_file(&ts_name,&ts_folder);
            cautomaton.automaton.draw(&cautomaton.name.0, &input_automata_system);
        }


        println!("\n Input automata drawing Done.\n");

        /////////////////////////////////////////////////////////////////////////////////
        /////////////////////////////////////////////////////////////////////////////////
        //Step 1: Synchronous product of the automata
        /////////////////////////////////////////////////////////////////////////////////
        /////////////////////////////////////////////////////////////////////////////////
        let sync_prod = match SyncProduct::build_sync_product(&self){
          Ok(sync_prod)=>sync_prod,
            Err(e)=>{
                println!("Error during the synchronous product of the automata: {}",e);
                return Err(CommunicatingAutomataCompositionError::SynchronousProductError);
            }
        };

        println!("Synchronous product of the automata successful.");
        sync_prod.draw(&name.to_string(), &step1_synchronous_product);
        println!("\nStep 1 Done. Synchronous product of the automata successful.\n");
        /////////////////////////////////////////////////////////////////////////////////
        /////////////////////////////////////////////////////////////////////////////////
        //Step 2: Global interaction derivation
        /////////////////////////////////////////////////////////////////////////////////
        /////////////////////////////////////////////////////////////////////////////////
        let automaton = sync_prod.to_automaton_string();
        let regex = match regex_algo{
            AutomataToRegexAlgo::Brzozowski=> {
                //Automaton::automaton_to_regex_brzozowski(&cautomaton.automaton)
                automaton.to_regex_brzozowski()
            },
            AutomataToRegexAlgo::StateElimination=> {
                automaton.to_regex_with_state_elimination().unwrap()
            },
            AutomataToRegexAlgo::TransitiveClosure=> {
                automaton.to_regex_with_transitive_closure().unwrap()
            }
            AutomataToRegexAlgo::ViaPetriNets=> {
                let files_folder = format!("{}/{}_derivation", output_folder, name);
                let _ = fs::create_dir_all(files_folder.clone());

                let aut = Self::preprocess_automaton_for_petrify_vp(&automaton);
                let reg = if let Ok((r,_,_)) = Automaton::automaton_to_regex_via_petri_nets(&aut,&name,files_folder.as_str(),true){
                    r
                }else{
                    return Err(CommunicatingAutomataCompositionError::RegexConversionIntoInteractionError);
                };
                Self::translate_regex_because_of_petrify_vp(&reg)
            }
        };

        let gen_ctx = self.extract_general_context()?;

        match Self::translate_regex_to_global_interaction_with_gates(&gen_ctx,&regex){
            Ok(global_interaction)=>{
                println!("Global Composition of Interaction Models Successful.");

                draw_model(&gen_ctx, name, &step2_global_interaction_derivation, &global_interaction);


                println!("\nStep 2 Done. Composition of the system of communicating automata successful.\n");


                Ok(global_interaction)
            }
            Err(e)=>{
                Err(CommunicatingAutomataCompositionError::RegexConversionIntoInteractionError)
            }
        }

    }

}


