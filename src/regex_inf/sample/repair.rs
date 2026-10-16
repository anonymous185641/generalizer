use std::collections::{HashMap, HashSet};
use petgraph::algo::toposort;
use petgraph::graph::{DiGraph, UnGraph};
use crate::regex_inf::algos::crx::CRX;
use crate::regex_inf::regex::regex::Regex;
use crate::regex_inf::sample::sample::Sample;



impl Sample{
    pub fn repair(v: Vec<String>, s: &Sample) -> Regex<String> {
        /* --------------------------------------------------
         * Line 1: construct S′ using PSubstring
         * -------------------------------------------------- */
        let s_prime = s.filter(v.clone());

        /* --------------------------------------------------
         * Line 2–3: empty CS → trivial OR-expression
         * -------------------------------------------------- */
        let (cs, ncs) = s_prime.tran_reduction();
        if cs.is_empty() {
            return Regex::Or(
                v.into_iter().map(Regex::Literal).collect()
            );
        }

        /* --------------------------------------------------
         * Line 5: build constraint graph G = Graph(CS(S′))
         * -------------------------------------------------- */
        let mut g = UnGraph::<String, ()>::new_undirected();
        let mut nodes = HashMap::new();

        for (a, b) in &cs {
            let na = *nodes.entry(a.clone()).or_insert_with(|| g.add_node(a.clone()));
            let nb = *nodes.entry(b.clone()).or_insert_with(|| g.add_node(b.clone()));
            g.add_edge(na, nb, ());
        }

        /* --------------------------------------------------
         * Line 6–8: clique removal → all MIS
         * -------------------------------------------------- */
        let mut all_mis: Vec<Vec<String>> = Vec::new();
        /*
        while g.node_count() > 0 {
            let mis = clique_removal(&mut g);
            all_mis.push(mis);
        }
         */
        while g.node_count()>0 {
            let (mis,cliq) = Self::clique_removal(&g);
            for label in mis.clone(){
                if let Some(index) = Self::get_node_index_from_weight(&g,&label){
                    g.remove_node(index);
                }
                else{
                    panic!("NodeIndex not found in graph: function merge");
                }
            }
            all_mis.push(mis);
        }

        /* --------------------------------------------------
         * Line 9–11: topological sort of each Mi
         * -------------------------------------------------- */
        let mut chains: Vec<Vec<String>> = Vec::new();

        for mi in &all_mis {
            let mut sub = DiGraph::<String, ()>::new();
            let mut idx = HashMap::new();

            for x in mi {
                idx.insert(x.clone(), sub.add_node(x.clone()));
            }

            for (a, b) in &cs {
                if idx.contains_key(a) && idx.contains_key(b) {
                    sub.add_edge(idx[a], idx[b], ());
                }
            }

            let topo = toposort(&sub, None).unwrap();
            chains.push(topo.into_iter().map(|n| sub[n].clone()).collect());
        }

        /* --------------------------------------------------
         * Line 12: extend chains using NCS
         * -------------------------------------------------- */
        let used: HashSet<_> = chains.iter().flatten().cloned().collect();
        for (a, b) in ncs {
            if !used.contains(&a) {
                let best = chains
                    .iter_mut()
                    .max_by_key(|c| c.len())
                    .unwrap();
                best.push(a);
            }
            if !used.contains(&b) {
                let best = chains
                    .iter_mut()
                    .max_by_key(|c| c.len())
                    .unwrap();
                best.push(b);
            }
        }

        /* --------------------------------------------------
         * Line 13: APPLY CRX (this is the non-obvious part)
         * -------------------------------------------------- */
        let mut repaired_chains = Vec::new();

        for chain in chains {
            //println!("Chain: {:?}",chain);
            let mut chain_sample = s_prime.filter(chain.clone());

            let r = CRX::crx(&chain_sample);

            repaired_chains.push(r);
        }

        /* --------------------------------------------------
         * Line 14–15: combine using interleaving
         * -------------------------------------------------- */
        if repaired_chains.len() == 1 {
            repaired_chains.remove(0)
        } else {
            Regex::Int(repaired_chains)
        }
    }
}