use petgraph::graph::NodeIndex;
use petgraph::data::DataMap;
use petgraph::dot::{Config, Dot};
use crate::regex_inf::regex::regex::Regex;
use crate::regex_inf::sample::sample::Sample;
use crate::regex_inf::soa::soa::SOA;

pub struct Soa2Chare{}


impl Soa2Chare{
    pub fn i_chare(sample: &Sample) ->Regex<String>{
        //The complete algo from the sample

        let mut soa = SOA::build_from_sample(sample);

        //println!("{:?}", Dot::with_config(&soa.soa, &[Config::EdgeNoLabel]));

        Self::soa2chare(&mut soa)
    }

    pub fn soa2chare(soa: &mut SOA) -> Regex<String>{
        let mut condensed = SOA::new();
        condensed = SOA::contract_with_alt(soa);

        //println!("{:?}", Dot::with_config(&condensed.soa, &[Config::EdgeNoLabel]));


        let level_order = condensed.build_level_order();
        /////////////////////////////////////////
        ////////////////////////////////////////

        let mut regex = Regex::Empty;

        let snk_order = level_order.get(&condensed.snk()).unwrap().clone();

        ///////////////////////////////////////////////////////////////////////////////////
        //For iterations, we convert into vector(to eliminate non determinism)
        let mut level_order_vec: Vec<(NodeIndex, usize)> = level_order.into_iter().collect();
        level_order_vec.sort_by_key(|&(node, _)| node);
        level_order_vec.dedup();


        ///////////////////////////////////////////////////////////////////////////////////

        for i in 1..=snk_order {

            let mut B : Vec<NodeIndex> = Vec::new();
            let mut C : Vec<NodeIndex> = Vec::new();

            for (nod,j) in &level_order_vec {

                if i == *j {

                    match condensed.soa[*nod] {
                        Regex::SNK => continue,
                        Regex::Plus(_) => {
                            B.push(*nod);
                        },
                        _ => {C.push(*nod);},
                    }

                }

            }

            for nod in B.clone() {
                if condensed.is_skip_level(i) || B.len() + C.len() >1 {
                    regex = Regex::concatenate(&regex, &Regex::question_mark(&condensed.soa[nod]));
                }
                else{
                    regex = Regex::concatenate(&regex,&condensed.soa[nod]);
                }


            }

            if C.len()> 0{
                // New label with alt
                let mut new_label = Regex::Empty;
                for node in C.clone(){
                    new_label =  Regex::or(&new_label,condensed.soa.node_weight(node).unwrap());
                }
                if condensed.is_skip_level(i) || B.len()>0 {

                    new_label = Regex::question_mark(&new_label);
                    //Concatenation
                    regex = Regex::concatenate(&regex,&new_label);
                }
                else {
                    regex = Regex::concatenate(&regex,&new_label);
                }
            }
        }

        regex

    }
}

