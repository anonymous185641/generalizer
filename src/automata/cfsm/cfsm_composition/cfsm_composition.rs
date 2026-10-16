use crate::automata::automaton::automaton::Automaton;
use crate::automata::cfsm::system_cfsm::system_cfsm::SystemCFSM;
use crate::automata::transition_systems::transition_systems_composition::ts_composition_error::TsCompositionError;
use crate::interactions::syntax::interaction::Interaction;

impl SystemCFSM {
    
    pub fn composition_to_global_interaction(&self,name: &str, target_folder: &str) -> Result<Interaction,TsCompositionError>{
        //Some preprocessing
        //Introduction of gates and empty transitions
        let system_with_gates = self.into_cfsm_with_gates().introduce_empty_transition();

        let input_folder = format!("{}/input_cfsm_system",target_folder);
        let _ = std::fs::create_dir_all(&input_folder);

        self.draw(&name.to_string(),&input_folder);
        //println!("System with gates:\n{:?}",self.machines);

        //let with_petri_folder = format!("{}/with_petri_nets",target_folder);
        //let _ = std::fs::create_dir_all(&with_petri_folder);

        /*
        let with_brzozowski_folder = format!("{}/with_brzozowski",target_folder);
        let _ = std::fs::create_dir_all(&with_brzozowski_folder);

        let with_transitive_closure_folder = format!("{}/with_transitive_closure",target_folder);
        let _ = std::fs::create_dir_all(&with_transitive_closure_folder);

        let with_state_elimination_folder = format!("{}/with_state_elimination",target_folder);
        let _ = std::fs::create_dir_all(&with_state_elimination_folder);

        let with_soa2chare_folder = format!("{}/with_soa2chare",target_folder);
        let _ = std::fs::create_dir_all(&with_soa2chare_folder);

        let with_soa2sore_folder = format!("{}/with_soa2sore",target_folder);
        let _ = std::fs::create_dir_all(&with_soa2sore_folder);

         */

         //Automaton::compose_system_of_transition_systems_via_petri_nets(&system_with_gates.machines, name, &target_folder)
        Automaton::compose_system_of_transition_systems_brzozowski(&system_with_gates.machines, name, &target_folder)
        //let _ = Automaton::compose_system_of_transition_systems_state_elim(&system_with_gates.machines, name, &with_state_elimination_folder);
        //let _ = Automaton::compose_system_of_transition_systems_transitive_closure(&system_with_gates.machines, name, &with_transitive_closure_folder);
        //let _ = Automaton::compose_system_of_transition_systems_soa2chare(&system_with_gates.machines, name, &with_soa2chare_folder);
         //Automaton::compose_system_of_transition_systems_soa2sore(&system_with_gates.machines, name, &with_soa2sore_folder)
    }
    
}


















































