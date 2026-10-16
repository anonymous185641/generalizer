use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use itertools::Itertools;
use petgraph::graph::NodeIndex;
use crate::regex_inf::soa::soa::SOA;

impl SOA {
    pub fn predicate_of_the_algo(&self)->Vec<NodeIndex>{
        let mut result:Vec<NodeIndex> = Vec::new();

        for node in self.nodes_with_only_src_as_predecessor(){
            if self.exclusive(&node) != Vec::from([node]){
                result.push(node);
            }
        }
        result
    }

    pub fn inclusion_max_reach_pair(&self)->Option<(NodeIndex,NodeIndex)>{
        let pairs = Self::subsets_of_size_2(&self.nodes_with_only_src_as_predecessor());

        let mut reach_sets_intersections = HashMap::new();

        for pair in pairs.clone(){
            let (a,b) = pair.into_iter().collect_tuple().unwrap();
            reach_sets_intersections.insert((a,b),Self::intersection(&self.reach(&a),&self.reach(&b)));

        }
        /*
        let f = reach_sets_intersections.iter()
            .max_by_key(|&(_, v)| v)
            .map(|(k, _)| k).clone();

         */

        match Self::inclusion_maximal_key(&reach_sets_intersections) {
            None => None,
            Some((u,v)) => Some((u.clone(),v.clone())),
        }
    }

    fn subsets_of_size_2<T: Clone>(items: &[T]) -> Vec<Vec<T>> {
        let mut result = Vec::new();

        for i in 0..items.len() {
            for j in (i + 1)..items.len() {
                result.push(vec![items[i].clone(), items[j].clone()]);
            }
        }

        result
    }

    fn inclusion_maximal_key(
        map: &HashMap<(NodeIndex, NodeIndex), Vec<NodeIndex>>,
    ) -> Option<(NodeIndex, NodeIndex)> {
        // Step 1: collect and canonicalize
        let mut entries: Vec<_> = map
            .iter()
            .map(|(k, v)| {
                let mut v = v.clone();
                v.sort_unstable();
                v.dedup();
                (k, v)
            })
            .collect();

        // Step 2: deterministic ordering
        entries.sort_by(|(k1, v1), (k2, v2)| {
            v1.len()
                .cmp(&v2.len())          // smaller first
                .then_with(|| k1.cmp(k2)) // deterministic tie-break
        });

        // Step 3: find inclusion-maximal value
        'outer: for (i, (key, val)) in entries.iter().enumerate() {
            for (j, (_, other)) in entries.iter().enumerate() {
                if i == j {
                    continue;
                }

                // other strictly contains val
                if val.len() < other.len()
                    && val.iter().all(|x| other.binary_search(x).is_ok())
                {
                    continue 'outer;
                }
            }

            return Some(**key);
        }

        None
    }


}