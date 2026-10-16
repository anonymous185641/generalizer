use petgraph::algo::all_simple_paths;
use petgraph::graph::NodeIndex;
use crate::regex_inf::soa::soa::SOA;

impl SOA {
    pub fn exclusive(&self, v: &NodeIndex) -> Vec<NodeIndex>{ //OK
        // description ...
        let mut result = Vec::new();
        let src = self.src();
        let snk = self.snk();

        let graph = self.soa.clone();
        // Generate all simple paths from source to each node
        for u in self.soa.node_indices() {
            if u == src ||  u== snk{ //u == *v ||
                continue; // Skip source and sink
            }

            let paths_from_src_to_u = all_simple_paths::<Vec<_>, _>(&graph, src, u,0, None).collect::<Vec<_>>();


            if paths_from_src_to_u.is_empty(){
                continue;
            }

            let mut ct =0;
            for path in  paths_from_src_to_u.clone(){
                if path.contains(&v){
                    ct+=1;
                }
            }
            if ct == paths_from_src_to_u.len(){
                result.push(u);
            }


        }
        //result.dedup();

        result
    }

}