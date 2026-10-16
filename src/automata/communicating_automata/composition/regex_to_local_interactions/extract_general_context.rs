use itertools::Itertools;
use crate::automata::communicating_automata::composition::caut_composition_error::CommunicatingAutomataCompositionError;
use crate::automata::communicating_automata::system_cautomata::cautomata::SystemCommunicatingAutomata;
use crate::interactions::syntax::general_context::GeneralContext;

impl SystemCommunicatingAutomata {

    pub fn extract_general_context(&self)->Result<GeneralContext, CommunicatingAutomataCompositionError>{
        let mut  general_context = GeneralContext::new();

        let mut lfs_names = Vec::new();
        let mut msgs = Vec::new();


        for aut_name in self.context.automata_names.iter(){
            lfs_names.push(aut_name.0.clone());
        }

        for msg_name in self.context.messages.iter(){
            msgs.push(msg_name.0.clone());
        }


        //lfs_names.sort();
        //lfs_names.dedup();

        //msgs.sort();
        //msgs.dedup();

        for lf_name in lfs_names.iter(){
            general_context.add_lf(lf_name.clone());
        }
        for msg in msgs.iter(){
            general_context.add_msg(msg.clone());
        }


        Ok(general_context)
    }

    /*
    pub fn extract_general_context(&self)->Result<GeneralContext,ConcurrentAutomataCompositionError>{
        let mut  general_context = GeneralContext::new();
        
        let mut labels = Vec::new();
        
        for automaton in self.automata.iter(){
            for edge_id in automaton.graph.edge_indices(){
                labels.push(automaton.graph.edge_weight(edge_id).unwrap().clone());
            }
        }

        let mut lfs_names = Vec::new();
        let mut msgs = Vec::new();
        for label in labels.iter().unique(){
            match ConcurrentAutomataAction::parse_label(label){
                Ok(action)=>{
                    match action{
                        ConcurrentAutomataAction::Emission(emission)=>{
                            lfs_names.push(emission.receiver.name.clone());
                            lfs_names.push(emission.emitter.name.clone());
                            msgs.push(emission.message.clone());
                            //general_context.add_lf(emission.receiver.name);
                            //general_context.add_lf(emission.emitter.name);
                            //general_context.add_msg(emission.message);
                        },
                        ConcurrentAutomataAction::Reception(reception)=>{
                            lfs_names.push(reception.emitter.name.clone());
                            lfs_names.push(reception.receiver.name.clone());
                            msgs.push(reception.message.clone());
                            //general_context.add_lf(reception.emitter.name);
                            //general_context.add_lf(reception.receiver.name);
                            //general_context.add_msg(reception.message);
                        }
                    }
                },
                Err(e)=>{
                    panic!("{}",e);
                }
            }
        }

        lfs_names.sort();
        lfs_names.dedup();

        msgs.sort();
        msgs.dedup();

        for lf_name in lfs_names.iter(){
            general_context.add_lf(lf_name.clone());
        }
        for msg in msgs.iter(){
            general_context.add_msg(msg.clone());
        }
        
        
        Ok(general_context)
    }

     */
    
    
}














