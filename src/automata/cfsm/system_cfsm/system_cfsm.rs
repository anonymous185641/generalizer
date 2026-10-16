use crate::automata::automaton::automaton::Automaton;

#[derive(Debug,Clone)]
pub struct SystemCFSM{
    pub machines: Vec<Automaton>
}


impl SystemCFSM{
    pub fn new() -> SystemCFSM{
        SystemCFSM{machines: Vec::new()}
    }
}











