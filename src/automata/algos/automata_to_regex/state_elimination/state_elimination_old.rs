use std::collections::HashMap;
use petgraph::stable_graph::{NodeIndex, StableGraph};
use petgraph::visit::{EdgeRef, IntoEdgeReferences};
use crate::automata::automaton::automaton::Automaton;
use crate::regex_inf::regex::regex::Regex;

type RGraph = StableGraph<(), Regex<String>>;


impl Automaton{

    fn add_or_merge_edge(
        g: &mut RGraph,
        s: NodeIndex,
        t: NodeIndex,
        label: Regex<String>,
    ) {
        if let Some(eid) = g.find_edge(s, t) {
            let w = g.edge_weight_mut(eid).unwrap();
            *w = Regex::or_vec_retain_non_empty(vec![w.clone(), label]);
        } else {
            g.add_edge(s, t, label);
        }
    }

    fn fuse_nodes(g: &mut RGraph, a: NodeIndex, b: NodeIndex) {
        if a == b { return; }

        // Collect edges before mutation
        let incoming: Vec<_> = g.edges_directed(b, petgraph::Incoming)
            .map(|e| (e.source(), e.weight().clone()))
            .collect();

        let outgoing: Vec<_> = g.edges_directed(b, petgraph::Outgoing)
            .map(|e| (e.target(), e.weight().clone()))
            .collect();

        // Redirect x -> b   into   x -> a
        for (src, w) in incoming {
            let real_src = if src == a { a } else { src };
            Self::add_or_merge_edge(g, real_src, a, w);
        }

        // Redirect b -> y   into   a -> y
        for (tgt, w) in outgoing {
            let real_tgt = if tgt == a { a } else { tgt };
            Self::add_or_merge_edge(g, a, real_tgt, w);
        }

        g.remove_node(b);
    }

    fn to_regex_graph(aut: &Automaton) -> (RGraph, NodeIndex, NodeIndex) {
        let mut g = RGraph::default();
        let mut map = HashMap::new();

        // 1️⃣ Copy nodes
        for n in aut.graph.node_indices() {
            let new = g.add_node(());
            map.insert(n, new);
        }

        // 2️⃣ Copy transitions (merge parallel with OR)
        for e in aut.graph.edge_references() {
            let s = map[&e.source()];
            let t = map[&e.target()];
            let label = Regex::Literal(e.weight().clone());
            Self::add_or_merge_edge(&mut g, s, t, label);
        }

        // 3️⃣ Fuse all initial states
        let new_initial = {
            let (&first, rest) = aut.initial_states
                .split_first()
                .expect("Automaton must have at least one initial state");

            let rep = map[&first];

            for other in rest {
                let other_idx = map[other];
                Self::fuse_nodes(&mut g, rep, other_idx);
            }
            rep
        };

        // 4️⃣ Fuse all final states
        let new_final = {
            let (&first, rest) = aut.final_states
                .split_first()
                .expect("Automaton must have at least one final state");

            let rep = map[&first];

            for other in rest {
                let other_idx = map[other];
                Self::fuse_nodes(&mut g, rep, other_idx);
            }
            rep
        };

        (g, new_initial, new_final)
    }

    /*Old
    fn to_regex_graph(aut: &Automaton) -> RGraph {
        let mut g = RGraph::default();
        let mut map = HashMap::new();


        for n in aut.graph.node_indices() {
            let new = g.add_node(());
            map.insert(n, new);
        }

        for e in aut.graph.edge_references() {
            let s = map[&e.source()];
            let t = map[&e.target()];
            let label = Regex::Literal(e.weight().clone());

            if let Some(edge) = g.find_edge(s, t) {
                let w = g.edge_weight_mut(edge).unwrap();
                *w = Regex::or_vec_retain_non_empty(vec![w.clone(), label]);
            } else {
                g.add_edge(s, t, label);
            }
        }

        g
    }

     */

    fn remove_state(g: &mut RGraph, q: NodeIndex) {
        let loop_r = g.find_edge(q, q)
            .map(|e| g.edge_weight(e).unwrap().clone())
            .unwrap_or(Regex::Empty);

        let loop_star = Regex::kleene_wrap(loop_r);

        let preds: Vec<_> = g.neighbors_directed(q, petgraph::Incoming).collect();
        let succs: Vec<_> = g.neighbors_directed(q, petgraph::Outgoing).collect();

        for &p in &preds {
            if p == q { continue; }
            let rpq = g.find_edge(p, q)
                .map(|e| g.edge_weight(e).unwrap().clone())
                .unwrap_or(Regex::Empty);

            for &s in &succs {
                if s == q { continue; }
                let rqs = g.find_edge(q, s)
                    .map(|e| g.edge_weight(e).unwrap().clone())
                    .unwrap_or(Regex::Empty);

                let through = Regex::concatenate_vec(vec![rpq.clone(), loop_star.clone(), rqs]);

                if let Some(edge) = g.find_edge(p, s) {
                    let existing = g.edge_weight(edge).unwrap().clone();
                    *g.edge_weight_mut(edge).unwrap() = Regex::or_vec_retain_non_empty(vec![existing, through]);
                } else {
                    g.add_edge(p, s, through);
                }
            }
        }

        g.remove_node(q);
    }



    pub fn dfa_to_regex_state_removal(aut: &Automaton) -> Regex<String> {
        let  (mut g,init_state,final_state) = Self::to_regex_graph(aut);

        let mut states: Vec<_> = g.node_indices().collect();

        // Keep only one start and one final ideally, but we can union later
       // let initial = states[0]; // assume one start for DFA
        //let final_state = *states.last().unwrap();

        for &q in &states.clone() {
            if q != init_state && q != final_state {
                Self::remove_state(&mut g, q);
            }
        }

        // Now only init and final should remain

        let r1 = Self::edge_or_empty(&g, init_state, init_state);   // loop on start
        let r2 = Self::edge_or_empty(&g, init_state, final_state);  // start -> final
        let r3 = Self::edge_or_empty(&g, final_state, init_state);  // final -> start
        let r4 = Self::edge_or_empty(&g, final_state, final_state); // loop on final

        // r1*
        let r1_star = Regex::kleene_wrap(r1);

        // r3 r1* r2
        let back_to_final = Regex::concatenate_vec(vec![
            r3,
            r1_star.clone(),
            r2.clone(),
        ]);

        // r4 + r3 r1* r2
        let middle = Regex::or_vec_retain_non_empty(vec![r4, back_to_final]);

        // (r4 + r3 r1* r2)*
        let middle_star = Regex::kleene_wrap(middle);

        // Final: r1* r2 ( ... )*
        Regex::concatenate_vec(vec![
            r1_star,
            r2,
            middle_star,
        ])

    }

    fn edge_or_empty(g: &RGraph, s: NodeIndex, t: NodeIndex) -> Regex<String> {
        if let Some(eid) = g.find_edge(s, t) {
            g.edge_weight(eid).unwrap().clone()
        } else {
            Regex::Empty
        }
    }
}
