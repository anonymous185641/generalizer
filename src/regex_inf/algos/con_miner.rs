use std::collections::{HashMap, HashSet};
use petgraph::algo::toposort;
use petgraph::Graph;
use petgraph::graph::{DiGraph, NodeIndex};
use crate::regex_inf::algos::crx::CRX;
use crate::regex_inf::regex::regex::Regex;
use crate::regex_inf::sample::sample::Sample;

pub struct ConMiner{}

impl ConMiner{
    /// Main entry point: Algorithm 1 (conMiner)

    pub fn con_miner(sample: &Sample) -> Regex<String> {
        // ------------------------------------------------------------
        // Step 1: transitive reduction
        // ------------------------------------------------------------
        let (constraint,l2) = sample.tran_reduction();

       // println!("const {:?}",constraint);
        // ------------------------------------------------------------
        // Step 2: build constraint graph (undirected)
        // ------------------------------------------------------------
        let mut g: Graph<String, (), petgraph::Undirected> = Graph::new_undirected();
        let mut node_map: HashMap<String, NodeIndex> = HashMap::new();

        let mut ensure_node = |s: &String, g: &mut Graph<String, (), petgraph::Undirected>| {
            if let Some(&idx) = node_map.get(s) {
                idx
            } else {
                let idx = g.add_node(s.clone());
                node_map.insert(s.clone(), idx);
                idx
            }
        };

        for (a, b) in &constraint {
            let u = ensure_node(a, &mut g);
            let v = ensure_node(b, &mut g);
            if u != v {
                g.add_edge(u, v, ());
            }
        }

        //println!("g {:?}",g);
        // ------------------------------------------------------------
        // Step 3: repeatedly extract MIS using clique_removal
        // ------------------------------------------------------------
        let mut allmis: Vec<Vec<String>> = Vec::new();


        while g.node_count()>0 {
            let (mut mis,cliq) = Sample::clique_removal(&g);
            for label in mis.clone(){
                if let Some(index) = Sample::get_node_index_from_weight(&g,&label){
                    g.remove_node(index);
                }
                else{
                    panic!("NodeIndex not found in graph: function merge");
                }
            }
            mis.sort();
            mis.dedup();
            allmis.push(mis);
        }
        // ------------------------------------------------------------
        // Step 4: add non-constraint symbols to first MIS
        // ------------------------------------------------------------
        let  alphabet_l2: HashSet<String> =
            l2.iter().flat_map(|(a, b)| vec![a.clone(), b.clone()]).collect();

        let alphabet_constraint: HashSet<String> =
            constraint.iter().flat_map(|(a, b)| vec![a.clone(), b.clone()]).collect();

        let non_constraint: Vec<String> = alphabet_l2
            .difference(&alphabet_constraint)
            .cloned()
            .collect();

        //println!("alphabet {:?}",non_constraint);
        /*
        if let Some(first) = allmis.first_mut() {
            for s in non_constraint {
                if !first.contains(&s) {
                    first.push(s);
                }
            }
            first.sort();
            first.dedup();
        }
         */
        if allmis.is_empty(){
            allmis.push(vec![]);
        }
        for s in non_constraint {
            if !allmis[0].contains(&s) {
                allmis[0].push(s);
            }
        }
        allmis[0].sort();
        allmis[0].dedup();
        //println!("all_mis {:?}",allmis);

        // ------------------------------------------------------------
        // Step 5: topological sort for each MIS
        // ------------------------------------------------------------
        let mut factors: Vec<Vec<String>> = Vec::new();

        for mis in &allmis {

            // directed graph induced by L2
            let mut dg: Graph<String, (), petgraph::Directed> = Graph::new();
            let mut dmap: HashMap<String, NodeIndex> = HashMap::new();

            let mut get = |s: &String, g: &mut Graph<String, (), petgraph::Directed>| {
                if let Some(&n) = dmap.get(s) {
                    n
                } else {
                    let n = g.add_node(s.clone());
                    dmap.insert(s.clone(), n);
                    n
                }
            };

            for s in mis {
                get(s, &mut dg);
            }

            for (a, b) in &l2 {
                if mis.contains(a) && mis.contains(b) {
                    let u = get(a, &mut dg);
                    let v = get(b, &mut dg);
                    if u != v {
                        dg.add_edge(u, v, ());
                    }
                }
            }

            let mut order: Vec<String> = toposort(&dg, None)
                .unwrap_or_default()
                .iter()
                .map(|n| dg[*n].clone())
                .collect();

            // stability
            order.sort_by(|a, b| a.cmp(b));
            factors.push(order);
        }
        //println!("Factors {:?}",factors);
        // ------------------------------------------------------------
        // Step 6: learner_oper = CRX
        // ------------------------------------------------------------
        let mut regex_factors: Vec<Regex<String>> = Vec::new();

        for factor in factors {

            let filtered = sample.filter(factor.clone());
            let r = CRX::crx(&filtered); // your existing CRX implementation
            regex_factors.push(r);

        }

        // ------------------------------------------------------------
        // Step 7: interleave all factors
        // ------------------------------------------------------------

        if regex_factors.len() == 1 {
            regex_factors.remove(0)
        } else {
            Regex::Int(regex_factors)
        }


    }

}