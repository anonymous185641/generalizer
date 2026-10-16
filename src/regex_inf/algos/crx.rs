use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use petgraph::algo::{kosaraju_scc, toposort};
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::prelude::EdgeRef;
use crate::regex_inf::regex::regex::Regex;
use crate::regex_inf::sample::sample::Sample;

pub struct CRX{}

impl CRX {


    pub fn crx(sample: &Sample) -> Regex<String> {
        let words = Self::parse_words(sample);
        if words.is_empty() {
            return Regex::Empty;
        }

        let precedence_graph = Self::build_precedence_graph(&words);
        let sccs = kosaraju_scc(&precedence_graph);
        let condensed = Self::build_scc_dag(&precedence_graph, &sccs);
        let mut topo = toposort(&condensed, None).unwrap();
        //topo.sort_by_key(|n| n.index()); // NOOOOOOOO, This is WROOOONG

        let block_infos = BlockInfo::analyze_blocks(
            &precedence_graph,
            &words,
            &sccs,
            &topo,
        );

        let mut parts = Vec::new();
        for block in block_infos {
            let mut literals: Vec<Regex<String>> = block
                .symbols
                .iter()
                .map(|s| Regex::Literal(s.clone()))
                .collect();

            let mut base = if literals.len() == 1 {
                literals.remove(0)
            } else {
                //Regex::Or(literals)
                literals.sort_by(|a, b| format!("{:?}", a).cmp(&format!("{:?}", b)));
                Regex::Or(literals)
            };

            if block.repeated {
                base = Regex::Plus(Box::new(base));
            }

            if block.optional {
                base = Regex::Qm(Box::new(base));
            }

            parts.push(base);
        }

        if parts.is_empty() {
            Regex::Epsilon
        } else if parts.len() == 1 {
            parts.remove(0)
        } else {
            Regex::Concat(parts)
        }
    }

    /* =========================
       Parsing
       ========================= */

    fn parse_words(sample: &Sample) -> Vec<Vec<String>> {
        sample
            .sample
            .iter()
            .map(|w| {
                if let Some(sep) = sample.separator {
                    w.split(sep).map(|s| s.to_string()).collect()
                } else {
                    w.chars().map(|c| c.to_string()).collect()
                }
            })
            .collect()
    }

    /* =========================
       Precedence graph
       ========================= */

    fn build_precedence_graph(words: &[Vec<String>]) -> DiGraph<String, ()> {
        let mut graph = DiGraph::<String, ()>::new();
        let mut nodes = HashMap::new();

        for word in words {
            for sym in word {
                nodes
                    .entry(sym.clone())
                    .or_insert_with(|| graph.add_node(sym.clone()));
            }
            for (a, b) in word.iter().tuple_windows() {
                let na = nodes[a];
                let nb = nodes[b];
                if na != nb {
                    graph.add_edge(na, nb, ());
                }
            }
        }

        graph
    }

    /* =========================
       SCC condensation
       ========================= */

    fn build_scc_dag(
        graph: &DiGraph<String, ()>,
        sccs: &[Vec<NodeIndex>],
    ) -> DiGraph<usize, ()> {
        let mut dag = DiGraph::<usize, ()>::new();
        let mut scc_nodes = Vec::new();

        for i in 0..sccs.len() {
            scc_nodes.push(dag.add_node(i));
        }

        let mut node_to_scc = HashMap::new();
        for (i, scc) in sccs.iter().enumerate() {
            for &n in scc {
                node_to_scc.insert(n, i);
            }
        }

        for edge in graph.edge_references() {
            let s = node_to_scc[&edge.source()];
            let t = node_to_scc[&edge.target()];
            if s != t {
                dag.add_edge(scc_nodes[s], scc_nodes[t], ());
            }
        }

        dag
    }
}


/* =========================
   Block analysis
   ========================= */

struct BlockInfo {
    symbols: Vec<String>,
    optional: bool,
    repeated: bool,
}

impl BlockInfo {
    fn analyze_blocks(
        graph: &DiGraph<String, ()>,
        words: &[Vec<String>],
        sccs: &[Vec<NodeIndex>],
        topo: &[NodeIndex],
    ) -> Vec<BlockInfo> {
        let mut blocks = Vec::new();

        for &node in topo {
            let idx = node.index();

            let mut symbols: Vec<String> = sccs[idx]
                .iter()
                .map(|n| graph[*n].clone())
                .collect();
            symbols.sort();
            symbols.dedup();

            let mut present_in = 0;
            let mut max_occ = 0;

            for w in words {
                let occ = w.iter().filter(|s| symbols.contains(*s)).count();
                if occ > 0 {
                    present_in += 1;
                }
                max_occ = max_occ.max(occ);
            }

            blocks.push(BlockInfo {
                symbols: symbols.into_iter().collect(),
                optional: present_in < words.len(),
                repeated: max_occ > 1,
            });
        }

        blocks
    }
}
