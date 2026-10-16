use petgraph::dot::{Config, Dot};
use crate::regex_inf::algos::soa2soire::Soa2Soire;
use crate::regex_inf::regex::regex::Regex;
use crate::regex_inf::sample::sample::Sample;

impl Sample{
    pub fn merge(&self)->Regex<String>{ //Garbage
        let mut graph = self.graph_from_cs();
        //println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));

        let mut result = Regex::Empty;
        let mut all_mis = Vec::new();


        while graph.node_count()>0 {
            let (mis,cliq) = Self::clique_removal(&graph);
            for label in mis.clone(){
                if let Some(index) = Self::get_node_index_from_weight(&graph,&label){
                    graph.remove_node(index);
                }
                else{
                    panic!("NodeIndex not found in graph: function merge");
                }
            }
            all_mis.push(mis);
        }
       // println!("ALL-MIS: {:?}", all_mis);

        for mis in all_mis{
            let new_sample = self.filter(mis.clone());

            let regex_mis = Soa2Soire::i_soire(&new_sample);
           // println!("mis: {:?}",&mis);
           // println!("new sample: {:?}",new_sample);
           // println!("regex mis: {:?}",regex_mis.to_text(None));
            result = Regex::interleave(&result,&regex_mis);
        }

        result

    }

}
