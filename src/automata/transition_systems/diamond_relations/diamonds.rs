use std::collections::HashSet;
use petgraph::stable_graph::NodeIndex;
use crate::automata::automaton::automaton::Automaton;
#[derive(Debug,Clone,Eq, PartialEq)]
pub struct AutomatonArc{
    pub source: NodeIndex,
    pub target: NodeIndex,
    pub label: String,
}

impl Automaton{
    pub fn are_in_basic_white_diamond(&self, arc1: &AutomatonArc, arc2: &AutomatonArc) ->bool{
        if arc1.label == arc2.label{ // First condition

            //We get the labels between source_1 and source_2
            let labels_s1_s2 = self.graph.edges_connecting(arc1.source,arc2.source).collect::<Vec<_>>();
            let mut labels_s1_s2_set = HashSet::new();
            for label in labels_s1_s2{
                labels_s1_s2_set.insert(label.weight().clone());
            }


            //We get the labels between target_1 and target_2
            let labels_t1_t2 = self.graph.edges_connecting(arc1.target,arc2.target).collect::<Vec<_>>();
            let mut labels_t1_t2_set = HashSet::new();
            for label in labels_t1_t2{
                labels_t1_t2_set.insert(label.weight().clone());
            }
            //Other conditions
            return (labels_s1_s2_set == labels_t1_t2_set) && (labels_s1_s2_set.len() > 0) && !labels_s1_s2_set.contains(&arc1.label);
        }

        false
    }

    pub fn are_in_white_diamond(&self,arc1: &AutomatonArc,arc2: &AutomatonArc)->bool{
        //White diamond is the smallest equivalence relation containing basic_white_diamond
        //Reflexivity and symmetry
        if (*arc1==*arc2) || self.are_in_basic_white_diamond(arc1,arc2) || self.are_in_basic_white_diamond(arc2,arc1){
            return true;
        }

        //Transitivity
       
        todo!()

    }

    pub fn white_diamond_class(&self,arc: &AutomatonArc)->Vec<AutomatonArc>{
        todo!()
    }

    pub fn are_in_black_diamond(arc1: &AutomatonArc,arc2: &AutomatonArc)->bool{
        todo!()
    }

    pub fn black_diamond_class(arc: &AutomatonArc)->Vec<AutomatonArc>{
        todo!()
    }

}






