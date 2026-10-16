use std::collections::HashSet;
use petgraph::algo::all_simple_paths;
use petgraph::Direction;
use petgraph::graph::NodeIndex;
use petgraph::prelude::EdgeRef;
use crate::regex_inf::soa::soa::SOA;

impl SOA{
    pub fn bend(&mut self){ //OK

        let src = self.src();
        let snk = self.snk();

        /* Find the set W of all elements that can be reached  from
        a predecessor of sink without crossing a successor of the source
         */

        let predecessors_of_snk = self.pred(&snk);
        let successors_of_src = self.succ(&src);

        //let mut W: HashSet<NodeIndex> = HashSet::new();
        let mut W: Vec<NodeIndex> = predecessors_of_snk.clone();

        for pred_snk in predecessors_of_snk{
            let reach_pred_snk = self.reach(&pred_snk);

            for nod in reach_pred_snk{
                let paths_from_pred_snk_to_nod = all_simple_paths::<Vec<_>, _>(&self.soa, pred_snk, nod,0, None).collect::<Vec<_>>();

                let mut ct:usize = 0;
                for path in paths_from_pred_snk_to_nod{
                    if Self::intersection(&successors_of_src,&path).len()>0{
                        ct+=1
                    }
                    /*
                    if successors_of_src.intersection(&HashSet::from_iter(path)).count() != 0{
                        ct+=1
                    }
                     */
                }
                if ct == 0 && nod!=pred_snk && nod != snk{
                    W.push(nod);
                }
                else if ct ==1 && successors_of_src.contains(&pred_snk){
                    W.push(nod);
                }
            }
        }
        // println!("{:?}",W);
        //Redirection
        for w in W{
            for edge in self.soa.clone().edges_directed(w,Direction::Outgoing){
                let (source,target) = self.soa.edge_endpoints(edge.id()).unwrap();

                if self.is_successor(&src,&target){
                    self.soa.remove_edge(edge.id());
                    self.soa.update_edge(w,snk,());
                }
            }
        }

    }
}