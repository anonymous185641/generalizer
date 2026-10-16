use std::fmt::Debug;
use std::fs::File;
use std::hash::Hash;
use std::io::Write;
use std::process::Command;
use crate::petri_nets::petri_net::petri_net::PetriNet;
use crate::petri_nets::petri_net::place_transition::PetriNetTransitionLabelBound;
use crate::petri_nets::petri_net_io::petri_net_parsing::petri_nets_string_parser::LiteralFromString;

impl<T: PetriNetTransitionLabelBound> PetriNet<T>{
    pub fn draw(&self,name:&String,folder:&String){
        let petri_net_dot_file_name = format!("{}/{}.dot", folder, name);
        let mut file = File::create(&petri_net_dot_file_name).unwrap();
        let _ = file.write(self.to_dot().as_bytes() );
        // ***
        let petri_net_picture_file_name = format!("{}/{}.png", folder, name);

        let _status = Command::new("dot")
            .arg("-Tpng")
            .arg(&petri_net_dot_file_name)
            .arg("-o")
            .arg(&petri_net_picture_file_name)
            .output();
    }
}