use std::path::PathBuf;
use crate::automata::communicating_automata::composition::benchmark::caut_benchmark_entry::PetrifyData;
use crate::petri_nets::petri_net::petri_net::{PetriNet, PetriNode};
use crate::petri_nets::petri_net_io::petri_net_parsing::petri_parsing_interface::petri_nets_parse_file;
use crate::regex_inf::regex::regex::Regex;


pub enum PetriNetToRegexError{
    OneSourcingError,
    AltIntSeparationError,
    RegexSynthesisError,
    InvalidFinalPetriNetError,
}


impl std::fmt::Display for PetriNetToRegexError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            PetriNetToRegexError::OneSourcingError => write!(f,"One sourcing error"),
            PetriNetToRegexError::AltIntSeparationError => write!(f,"Separation of alternatives and interleaving error"),
            PetriNetToRegexError::RegexSynthesisError => write!(f,"Regular expression synthesis error"),
            PetriNetToRegexError::InvalidFinalPetriNetError => write!(f,"A final Petri net cannot be converted into a regular expression.",),
        }
    }
}


impl PetriNet<Regex<String>>{

    pub fn petri_to_regex(&self,name: &str, target_folder:Option<&str>,verbose:bool,draw:bool)->Result<Regex<String>,PetriNetToRegexError>{

        //Output folder preparation

        let output_dir = if let Some(target) = target_folder{
            format!("{}/conversion_petri_net_to_regex_output",target)
        }
        else{
            "conversion_petri_net_to_regex_output".to_string()
        };
        //let _ = std::fs::remove_dir_all(&output_dir);
        let _ = std::fs::create_dir_all(&output_dir);

        let step1_dir = format!("{}/step1_one_sourcing",output_dir);
        let _ = std::fs::create_dir_all(&step1_dir);

        let step2_dir = format!("{}/step2_alt_int_separation",output_dir);
        let _ = std::fs::create_dir_all(&step2_dir);

        let step3_dir = format!("{}/step3_regex_synthesis",output_dir);
        let _ = std::fs::create_dir_all(&step3_dir);

        let starting_petri_net_dir = format!("{}/starting_petri_net",output_dir);
        let _ = std::fs::create_dir_all(&starting_petri_net_dir);
        
        
        //We try retrieve the name of the input file without the extension
        let name_without_extension = if let Some(name) = PathBuf::from(name).file_stem().unwrap().to_str(){
            name.to_string()
        }
        else{
            name.to_string()
        };

        //Drawing the starting Petri net
        if draw{
            self.draw(&name_without_extension,&starting_petri_net_dir);
        }
       


        //Step 1
        /*
        if verbose{
            println!(
                "\n-----------------------------------------------------------------------\n\
                     One Sourcing of the Petri net {} \n\
                    -----------------------------------------------------------------------\n",name
            );
        }

        let mut one_sourced_net = self.clone();
        one_sourced_net.one_sourcing();


        //one_sourced_net.one_ending();

        one_sourced_net.draw(&name_without_extension,&step1_dir);
        if verbose{
            println!("One sourcing of the Petri net {} successful",name);
            println!("The output of this step is in the following folder: {}",step1_dir);
        }
        */

        //Step 2
        //NO ALT-INT-SEPARATION ANYMORE
        /*
        if verbose{
            println!(
                "\n-----------------------------------------------------------------------\n\
                     Separation of alternatives and interleaving in the Petri net {} \n\
                    -----------------------------------------------------------------------\n",name
            );
        }


        let step2_petri_net = if let Some(p) =
            one_sourced_net.alt_int_separation(&"alt_interleaving_separation_logger".to_string(),&step2_dir).first(){
            p.clone()
        }
        else{
            if verbose{
                println!("Failure of Separation of alternatives and interleaving in the Petri net {}",name);
            }

            return Err(PetriNetToRegexError::AltIntSeparationError);
        };

        step2_petri_net.draw(&name_without_extension,&step2_dir);

        if verbose{
            println!("The output of this step is in the following folder: {}",step2_dir);
        }

        */


        //Step 3
        if verbose{
            println!(
                "\n-----------------------------------------------------------------------\n\
                    Regular expression synthesis for the Petri net {} \n\
                    -----------------------------------------------------------------------\n",name
            );
        }

        ///////////////////////////////
        //////////////////////////////
        //With alt-int separation
        //let results =  step2_petri_net.petri_to_regex_synthesis(&"regex_synthesis_derivation".to_string(),&step3_dir);
        //No alt-int separation anymore, so we une the one-sourced net
        //let results =  one_sourced_net.petri_to_regex_synthesis(&"regex_synthesis_derivation".to_string(),&step3_dir);

        //No one-sourcing anymore, so we use the original net
        let results =  self.petri_to_regex_synthesis(&"regex_synthesis_derivation".to_string(),&step3_dir,draw);

        for petri in results{
            if let Ok(regex) = petri.get_regex_from_final_petri_net(){
                if draw{
                    petri.draw(&name_without_extension,&step3_dir);
                }
               
                if verbose{
                    println!("The output of this step is in the following folder: {}",step3_dir);
                }
                return Ok(regex);
            }
        }

        if verbose{
            println!("Failure of Regular expression synthesis for the Petri net {}",name);
        }

        return Err(PetriNetToRegexError::RegexSynthesisError);
    }



    pub fn get_regex_from_final_petri_net(&self)->Result<Regex<String>,PetriNetToRegexError>{
        /*
        if self.transitions.len() == 1
        && self.places.len() == 2
        && self.initial_marking.tokens.len()==1{
            /*
            let transition_index = self.get_node_index(&PetriNode::Transition(self.transitions[0].clone())).unwrap();
            if let PetriNode::Transition(t) = self.graph[transition_index].clone(){
                return Ok(t.label);
            }
             */

            return Ok(self.transitions[0].label.clone().simplify());
        }

        return Err(PetriNetToRegexError::InvalidFinalPetriNetError);
    }
         */

        if self.transitions.len()==1{

            let transition = self.transitions[0].clone();
            let preset_paces = self.preset_transition(&transition);
            for p in preset_paces{
                //We check that each preset place have exactly one token
                if !self.initial_marking.tokens.contains_key(&p){
                    return Err(PetriNetToRegexError::InvalidFinalPetriNetError);
                }
                if self.initial_marking.tokens[&p].0 <1{
                    return Err(PetriNetToRegexError::InvalidFinalPetriNetError);
                }
            }
            return Ok(self.transitions[0].label.clone().simplify());
        }else{
            return Err(PetriNetToRegexError::InvalidFinalPetriNetError);
        }
    }
}





















