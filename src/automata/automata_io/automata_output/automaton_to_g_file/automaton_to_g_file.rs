use itertools::Itertools;
use petgraph::data::DataMap;
use crate::automata::automaton::automaton::Automaton;

impl  Automaton {

    pub fn to_g_file_string(&self)->String{

        let mut transition_labels = self.graph.edge_weights().collect::<Vec<_>>();
        transition_labels.sort();
        transition_labels.dedup();

        let mut g_file_string = String::new();

        g_file_string.push_str(".outputs");
        g_file_string.push_str("\n");

        g_file_string.push_str(".dummy ");
        for label in transition_labels.iter(){
            g_file_string.push_str(&format!("{} ",label));
        }

        g_file_string.push_str("\n");
        g_file_string.push_str(".state graph");
        g_file_string.push_str("\n");
        for edge in self.graph.edge_indices(){
            let (source_id,target_id) = self.graph.edge_endpoints(edge).unwrap();

            let source_label = self.graph.node_weight(source_id).unwrap();
            let target_label = self.graph.node_weight(target_id).unwrap();
            let transition_label = self.graph.edge_weight(edge).unwrap();

            g_file_string.push_str(&format!("{} {} {}\n",source_label,transition_label,target_label));
        }


        if self.initial_states.len()>0{

            g_file_string.push_str(".marking {");
            if self.initial_states.len()==1{

                let state_label = self.graph.node_weight(self.initial_states[0]).unwrap();
                g_file_string.push_str(&format!("{}",state_label));
                g_file_string.push_str("}");
            }
            else{
                for state in self.initial_states.iter(){
                    let state_label = self.graph.node_weight(*state).unwrap();
                    g_file_string.push_str(&format!(" {}",state_label));
                }
                g_file_string.push_str(" }");
            }


        }

        g_file_string.push_str("\n");
        g_file_string.push_str(".end");
        g_file_string.push_str("\n");

        g_file_string

    }
    pub fn to_g_file(&self,name: &str,folder:&str){
        let file_name = format!("{}/{}.g",folder,name);
        let g_file_string = self.to_g_file_string();
        std::fs::write(&file_name,g_file_string).unwrap();
        //println!("File {} written",file_name);
    }
}










