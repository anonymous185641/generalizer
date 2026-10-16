use std::collections::HashMap;
use petgraph::{algo, Direction};
use petgraph::algo::{all_simple_paths, toposort};
use petgraph::graph::NodeIndex;
use petgraph::prelude::EdgeRef;
use crate::regex_inf::soa::soa::SOA;

impl SOA {

    pub fn build_level_order(&self) -> HashMap<NodeIndex, usize> {
        let graph = &self.soa;

        // Topological order (fails if cycles remain)
        let topo = toposort(graph, None)
            .expect("SOA must be acyclic before computing levels");

        let mut level: HashMap<NodeIndex, usize> = HashMap::new();

        // Initialize all levels to 0
        for &v in &topo {
            level.insert(v, 0);
        }

        // Longest-path DP
        for &u in &topo {
            let u_level = level[&u];

            for edge in graph.edges_directed(u, Direction::Outgoing) {
                let v = edge.target();
                let candidate = u_level + 1;

                let entry = level.entry(v).or_insert(0);
                if candidate > *entry {
                    *entry = candidate;
                }
            }
        }

        level
    }

    pub fn is_skip_level(&self, level: usize) -> bool {
        let levels = self.build_level_order();

        for edge in self.soa.edge_references() {
            let u = edge.source();
            let v = edge.target();

            let lu = match levels.get(&u) {
                Some(l) => *l,
                None => continue,
            };

            let lv = match levels.get(&v) {
                Some(l) => *l,
                None => continue,
            };

            // check: lu < level < lv
            if lu < level && level < lv {
                return true;
            }
        }

        false
    }


    pub fn build_level_order_old(&self)->HashMap<NodeIndex,usize>{
        //Build the level order of the soa

        let mut level_order :HashMap<NodeIndex,usize> = HashMap::new();

        let mut ret: Vec<Vec<NodeIndex>> = Vec::new();
        let mut counter = 1;

        //src has level 0
        ret.push(vec![self.src()]);


        loop {
            let mut current_level: Vec<NodeIndex> = Vec::new();

            for nod in ret[counter-1].iter(){
                for e in self.soa.edges_directed(*nod,petgraph::Direction::Outgoing) {
                    if  !current_level.contains(&e.target()) && e.source()!=e.target() {
                        current_level.push(e.target());
                    }
                }
            }
            let ret_2 = ret.clone();
            ret.push(current_level.clone());


            let mut tot_vec = Vec::new();
            for it in ret_2{
                let mut it_2 = it.clone();
                it_2.dedup();
                tot_vec.append(&mut it_2);

            }
            tot_vec.dedup();
            if tot_vec.len() >= self.soa.node_count(){
                break;
            }
            counter+=1;

        }

        let mut ct :usize= 0;
        for item in ret{
            for nod in item{

                level_order.insert(nod, ct);

            }
            ct+=1
        }

        level_order
    }

    pub fn is_skip_level_old(&self,level: usize)-> bool{
        //Determines if a given level  is a skip level or not
        /*Checks if a given level order is a skip level.

        We remind that a level i is a skip level if there exists nodes u and v
        with level order j_u < i and j_v > i such that there is a path from u to v.

         */
        let level_order = self.build_level_order();
        for edge in self.soa.edge_indices(){
            let (source,target) = self.soa.edge_endpoints(edge).unwrap();

            //We get the level of the source and target nodes
            if let (Some(source_level_order),Some(target_level_order)) = (level_order.get(&source),level_order.get(&target)){
                //println!("{} {}",source_level_order,target_level_order);
                if *source_level_order< level && *target_level_order> level { //We check the skip level condition
                    return true;
                }
            }

        }

        false
    }
}