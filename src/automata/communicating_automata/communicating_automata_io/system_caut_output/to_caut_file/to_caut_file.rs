use std::fmt::format;
use crate::automata::communicating_automata::system_cautomata::cautomata::SystemCommunicatingAutomata;

impl SystemCommunicatingAutomata{
    pub fn to_caut_file_string(&self)->String{

        let mut file_string = String::new();

        ///////////////////////////
        //Header
        //////////////////////////

        //@automata line
        file_string.push_str("@automata{");
        for automaton_name in self.context.automata_names.iter(){
            file_string.push_str(&format!("{};",automaton_name.0));
        }
        file_string.pop();//The remove the last comma
        file_string.push_str("}");
        file_string.push('\n');

        //@messages line
        file_string.push_str("@messages{");
        for message in self.context.messages.iter(){
            file_string.push_str(&format!("{};",message.0));
        }
        file_string.pop();
        file_string.push_str("}");
        file_string.push('\n');

        //@gates line
        file_string.push_str("@gates{");
        for gate in self.context.gates.iter(){
            file_string.push_str(&format!("{};",gate.0));
        }
        file_string.pop();
        file_string.push_str("}");
        file_string.push_str("\n\n");

        /////////////////////////
        //Automata
        ////////////////////////
        for automaton in self.automata.iter(){
            //Automaton name
            file_string.push_str(&format!(".automaton {}\n",automaton.name.0));

            //Edges
            for edge in automaton.automaton.graph.edge_indices(){
                let (src,tgt) = automaton.automaton.graph.edge_endpoints(edge).unwrap();

                let edge_label = automaton.automaton.graph.edge_weight(edge).unwrap();

                let src_name = automaton.automaton.graph.node_weight(src).unwrap();
                let tgt_name = automaton.automaton.graph.node_weight(tgt).unwrap();

                file_string.push_str(&format!("{} {} {}\n",src_name,edge_label,tgt_name));

            }
            //Initial state
            //To do: exception handling
            let initial_state = automaton.automaton.initial_states.iter().next().unwrap();
            let initial_state_name = automaton.automaton.graph.node_weight(*initial_state).unwrap();

            file_string.push_str(&format!(".initial state {{{}}}\n",initial_state_name));

            //Final states
            file_string.push_str(".final states {");
            for final_state in automaton.automaton.final_states.iter(){
                file_string.push_str(&format!("{};",automaton.automaton.graph.node_weight(*final_state).unwrap()));
            }
            file_string.pop();
            file_string.push_str("}\n");

            //.end
            file_string.push_str(".end\n\n");
        }

        file_string
    }

    pub fn to_caut_file(&self, name:&str, target_folder:Option<&str>){
        if target_folder.is_some() && !std::path::Path::new(target_folder.unwrap()).exists() {
            println!("Creating folder {}",target_folder.unwrap());
            std::fs::create_dir_all(target_folder.unwrap()).unwrap();
        }

        let g_file_string = self.to_caut_file_string();
        let file_name = if let Some(folder) = target_folder {
            format!("{}/{}.caut",folder,name)
        } else{
            format!("{}.caut",name)
        };
        std::fs::write(&file_name,g_file_string).unwrap();

        println!("File {} written",file_name);
    }

}
























