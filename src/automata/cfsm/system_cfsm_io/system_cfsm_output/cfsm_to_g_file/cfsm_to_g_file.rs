use crate::automata::cfsm::system_cfsm::system_cfsm::SystemCFSM;

impl SystemCFSM {
    pub fn to_g_file_string(&self)->Vec<String>{
        let mut g_file_string = Vec::new();
        for system in self.machines.iter(){
            g_file_string.push(system.to_g_file_string());
        }

        g_file_string
    }

    pub fn to_g_file(&self,name: &str,folder:&str){
        let g_file_string = self.to_g_file_string();
        for (i,g_file) in g_file_string.iter().enumerate(){
            let file_name = format!("{}/{}_{}.g",folder,name,i);
            std::fs::write(&file_name,g_file).unwrap();
            println!("File {} written",file_name);
        }
    }
}


























