use crate::automata::cfsm::system_cfsm::system_cfsm::SystemCFSM;

impl SystemCFSM {

    pub fn draw(&self,name:&String,folder:&String) {

        for (i,machine) in self.machines.iter().enumerate() {
            machine.draw(&format!("{}_M{}",name,i),folder);
        }
    }


}