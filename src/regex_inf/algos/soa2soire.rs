use std::collections::HashSet;
use petgraph::algo::kosaraju_scc;
use petgraph::dot::{Config, Dot};
use crate::regex_inf::regex::regex::Regex;
use crate::regex_inf::sample::sample::Sample;
use crate::regex_inf::soa::soa::SOA;

pub struct Soa2Soire {}

impl Soa2Soire{
    pub fn i_soire(sample: &Sample) ->Regex<String>{
        //The complete algo from the sample
        let mut soa = SOA::build_from_sample(sample);

        //println!("SOA: {:?}",Dot::with_config(&soa.soa, &[Config::EdgeNoLabel]));

        Self::soa2soire(&mut soa,sample)
    }

    pub fn soa2soire (soa: &mut SOA,given_sample: &Sample) -> Regex<String> {

        if soa.soa.edge_count() == 0 {
            return Regex::Empty;
        }
        else if soa.soa.node_count() == 2{
            return Regex::Epsilon;
        }
        else if soa.has_cycle() {

           // let components = kosaraju_scc(&soa.soa);
            let component = soa.valid_stcc().unwrap();
            //println!("picked component: {:?}", component);

            if component.len()==1 {
                soa.fuse_nodes(&component,Regex::Plus(Box::new(soa.soa.node_weight(component[0]).unwrap().clone())));
            }
            else{

                let mut label_component = Vec::new();
                for nod in component.clone(){
                    if let Some(Regex::Literal(s)) = soa.soa.node_weight(nod){
                        label_component.push(s.clone());
                    }
                }
                let new_sample = given_sample.filter(label_component);
                soa.fuse_nodes(&component,new_sample.merge());
            }

        }
        else if !SOA::are_vectors_equals(&soa.succ(&soa.src()), &soa.nodes_with_only_src_as_predecessor()){
        //else if soa.succ(&soa.src()) != soa.nodes_with_only_src_as_predecessor(){
            soa.add_epsilon();

        }
        else if soa.nodes_with_only_src_as_predecessor().len() == 1{

            let soa_clone = soa.clone();
            let src_successor =soa.nodes_with_only_src_as_predecessor()[0].clone();
            let label_1 = soa_clone.soa.node_weight(src_successor).unwrap();

            // println!("Before {:?}",soa.soa);
            soa.fuse_nodes(&vec![soa.src(),src_successor],Regex::SRC);
            //println!("After {:?}",soa.soa);

            let label_2 = Self::soa2soire(soa,given_sample);
            return Regex::concatenate(&label_1,&label_2);

        }
        else if soa.predicate_of_the_algo().len()>0{

            let mut nodes = soa.predicate_of_the_algo();

            let u = soa.exclusive(&nodes[0]);

            soa.fuse_nodes(&u, Self::soa2soire(&mut soa.extract(&u),given_sample));

        }
        else {
            if let Some((u,v)) = soa.inclusion_max_reach_pair(){
                soa.fuse_nodes(&Vec::from([u,v]),Regex::or(&soa.soa.node_weight(u).unwrap(),&soa.soa.node_weight(v).unwrap()));
            }
            else{
                panic!()
            }
        }

         Self::soa2soire(soa,given_sample)

    }
}



