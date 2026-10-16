use std::fs::File;
use std::io::Write;
use std::process::Command;
use graphviz_dot_builder::traits::DotTranslatable;
use crate::automata::automaton::automaton::Automaton;

impl Automaton{
    pub fn draw(&self,name:&String,folder:&String){
        let automaton_dot_file_name = format!("{}/{}.dot", folder, name);
        let mut file = File::create(&automaton_dot_file_name).unwrap();
        let _ = file.write(self.to_dot().to_dot_string().as_bytes() );
        // ***
        let automaton_picture_file_name = format!("{}/{}.png", folder, name);

        let _status = Command::new("dot")
            .arg("-Tpng")
            .arg(&automaton_dot_file_name)
            .arg("-o")
            .arg(&automaton_picture_file_name)
            .output();
    }
    
    
}














