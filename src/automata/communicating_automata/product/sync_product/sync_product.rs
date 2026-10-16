use std::collections::{BTreeSet, VecDeque};
use petgraph::data::DataMap;
use petgraph::Direction;
use petgraph::prelude::StableGraph;
use petgraph::stable_graph::NodeIndex;
use crate::automata::communicating_automata::product::sync_product::state::SyncProductState;
use crate::automata::communicating_automata::system_cautomata::cautomata::{CommunicatingAutomaton, SystemCommunicatingAutomata};
use crate::automata::communicating_automata::system_cautomata::cautomata_actions::{CommunicatingAutomataAction, CommunicatingAutomataValuePassing};
use crate::automata::communicating_automata::system_cautomata::cautomata_context::SystemCommunicatingAutomataContext;



pub struct SyncProduct {
    pub graph: StableGraph<SyncProductState, CommunicatingAutomataValuePassing>,
    pub initial_state: NodeIndex,
    pub final_states: Vec<NodeIndex>,
    pub name: String,
    pub context: SystemCommunicatingAutomataContext
}

impl SyncProduct {

    pub fn new(graph: StableGraph<SyncProductState, CommunicatingAutomataValuePassing>,
               initial_state: NodeIndex,final_states: Vec<NodeIndex>,name: String, context: SystemCommunicatingAutomataContext) -> SyncProduct {
        SyncProduct{graph,initial_state,final_states,name,context}
    }

    pub fn build_sync_product(system: &SystemCommunicatingAutomata) -> Result<SyncProduct, String> {

        let initial_state = SyncProductState::get_initial_state(system);

        let mut queue:VecDeque<CommunicatingAutomataAction> = VecDeque::new();

        let mut graph = StableGraph::new();
        let init_node_index = graph.add_node(initial_state.clone());
        let mut sync_prod = SyncProduct::new(graph, init_node_index, Vec::new(), system.system_name.clone(), system.context.clone());


        let mut exploration_space = BTreeSet::from([(None,initial_state.clone())]);
        let mut frontier = BTreeSet::from([initial_state]);

        while let Ok(next_global_states) = Self::unexplored_states(system,&frontier){
            //println!("TATATARA!!!!");
            //println!("{:?}",next_global_states);
            frontier.clear();
            if next_global_states.is_empty(){
                break;
            }

            for (state1,vp,state2) in next_global_states.iter(){

                if !exploration_space.contains(&(Some(vp.clone()),state2.clone())){
                    frontier.insert(state2.clone());
                    exploration_space.insert((Some(vp.clone()),state2.clone()));

                    sync_prod.graph.add_node(state2.clone());
                    let node_index1 = sync_prod.get_node_index(&state1).unwrap();
                    let node_index2 = sync_prod.get_node_index(&state2).unwrap();
                    sync_prod.graph.add_edge(node_index1,node_index2,vp.clone());
                    frontier.insert(state2.clone());
                }

            }
        }

        sync_prod.final_states =  SyncProductState::get_final_states(system,&sync_prod.graph);

       Ok(sync_prod)
    }

    pub fn unexplored_states(system: &SystemCommunicatingAutomata, frontier: &BTreeSet<SyncProductState>)
        ->Result<BTreeSet<(SyncProductState, CommunicatingAutomataValuePassing,SyncProductState)>,String>{
        let mut next_frontier = BTreeSet::new();

        for state in frontier.iter(){
            let new_f = Self::global_frontier(system,&state)?;
            for (new_vp,s) in new_f.iter(){
                next_frontier.insert((state.clone(),new_vp.clone(),s.clone()));
            }

        }

        Ok(next_frontier)
    }
    pub fn global_frontier(system: &SystemCommunicatingAutomata, state: &SyncProductState)->Result<BTreeSet<(CommunicatingAutomataValuePassing,SyncProductState)>,String>{
        let mut frontier = BTreeSet::new();

        for (id_send,node_send) in state.state.iter().enumerate(){
            let local_frontier_send = SyncProduct::local_frontier(&system.automata[id_send], *node_send)?;

            for (action_send,successor_send) in local_frontier_send.iter(){
                if let CommunicatingAutomataAction::Emission(emission) = action_send {
                    //Emission in the queue
                    //We search the corresponding reception in another state

                    for (id_recv,node_recv) in state.state.iter().enumerate(){
                        if id_recv == id_send{
                            continue;
                        }

                        let local_frontier_recv = SyncProduct::local_frontier(&system.automata[id_recv], *node_recv)?;

                        for (action_recv,successor_recv) in local_frontier_recv.iter(){
                            if let CommunicatingAutomataAction::Reception(reception) = action_recv {
                                if CommunicatingAutomataAction::are_complementary(action_send,action_recv){
                                    //We have our value passing
                                    //We compute the new state
                                    let mut new_state = state.clone();
                                    new_state.state[id_send] = successor_send.clone();
                                    new_state.state[id_recv] = successor_recv.clone();
                                    frontier.insert((CommunicatingAutomataValuePassing{emission:emission.clone(),reception:reception.clone()},new_state));

                                }
                            }
                        }

                    }
                }
            }

        }


        Ok(frontier)
    }

    pub fn local_frontier(caut: &CommunicatingAutomaton, node: NodeIndex) -> Result<Vec<(CommunicatingAutomataAction,NodeIndex)>,String> {
        let mut results : Vec<(CommunicatingAutomataAction,NodeIndex)> = Vec::new();

        let mut outgoing_neighbors = caut.automaton.graph.neighbors_directed(node, Direction::Outgoing).collect::<Vec<_>>();
         outgoing_neighbors.sort();
        outgoing_neighbors.dedup();

        for neighbor in outgoing_neighbors.iter() {

            for edge in caut.automaton.graph.edge_indices(){
                let (src,tgt) = caut.automaton.graph.edge_endpoints(edge).unwrap();

                if src == node  && tgt == *neighbor {
                    let label = caut.automaton.graph.edge_weight(edge).unwrap();
                    let action = CommunicatingAutomataAction::parse_label(&label)?;
                    results.push((action, neighbor.clone()));
                }

            }

        }

        Ok(results)
    }



    pub fn consume_action(system: &SystemCommunicatingAutomata,action:CommunicatingAutomataAction,state: &SyncProductState)->Option<SyncProductState>{
        todo!()
    }

    pub fn get_node_index(&self,node_weight: &SyncProductState)->Option<NodeIndex>{
        for node in self.graph.node_indices(){
            if self.graph[node]==*node_weight{
                return Some(node);
            }
        }

        None
    }

    pub fn get_node_weight(&self,node_index: &NodeIndex)->Option<SyncProductState>{
        for node in self.graph.node_indices(){
            if node==*node_index{
                return Some(self.graph[node].clone());
            }
        }
        None
    }

}















