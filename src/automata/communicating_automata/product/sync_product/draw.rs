use std::fs::File;
use std::io::Write;
use std::process::Command;
use graphviz_dot_builder::edge::edge::GraphVizEdge;
use graphviz_dot_builder::edge::style::GraphvizEdgeStyleItem;
use graphviz_dot_builder::graph::graph::GraphVizDiGraph;
use graphviz_dot_builder::graph::style::{GraphvizGraphStyleItem, GvGraphRankDir};
use graphviz_dot_builder::item::node::node::GraphVizNode;
use graphviz_dot_builder::item::node::style::{GraphvizNodeStyleItem, GvNodeShape};
use graphviz_dot_builder::traits::{DotBuildable, DotTranslatable};
use crate::automata::automaton::automaton::Automaton;
use crate::automata::communicating_automata::product::sync_product::sync_product::SyncProduct;

impl SyncProduct{

    pub fn draw(&self,name:&String,folder:&String){
        let automaton_dot_file_name = format!("{}/{}.dot", folder, name);
        let mut file = File::create(&automaton_dot_file_name).unwrap();
        let _ = file.write(self.to_dot().to_dot_string().as_bytes() );
        // ***
        let automaton_picture_file_name = format!("{}/{}.png", folder, name);

        let _status = Command::new("dot")
            .arg("-Tpng")
            .arg(&automaton_dot_file_name)
            .arg("-o")
            .arg(&automaton_picture_file_name)
            .output();
    }

    pub fn to_dot(&self)->GraphVizDiGraph{
        let mut new_graph = GraphVizDiGraph::new(vec![GraphvizGraphStyleItem::Rankdir(GvGraphRankDir::LR)]);

        //Starting node
        let init = GraphVizNode::new("init".to_string(), vec![GraphvizNodeStyleItem::Shape(GvNodeShape::Point)]);
        new_graph.add_node(init.clone());
        //Nodes
        for node in self.graph.node_indices(){
            if self.final_states.contains(&node){
                let new_node = GraphVizNode::new(self.get_node_weight(&node).unwrap().to_string(),
                                                 vec![GraphvizNodeStyleItem::Shape(GvNodeShape::DoubleCircle)] );
                //GraphvizNodeStyleItem::Label(node.index().to_string())
                new_graph.add_node(new_node);
            }
            else {
                //  let new_node = GraphVizNode::new(node.index().to_string(), vec![GraphvizNodeStyleItem::Shape(GvNodeShape::Circle)] );
                let new_node = GraphVizNode::new(self.get_node_weight(&node).unwrap().to_string(),
                                                 vec![GraphvizNodeStyleItem::Shape(GvNodeShape::Circle)] );
                new_graph.add_node(new_node);
                //GraphvizNodeStyleItem::Label(node.index().to_string()),
            }
        }

        //Initial edge
        let initial_edge = GraphVizEdge::new(init.id, None, self.get_node_weight(&self.initial_state).unwrap().to_string(), None, vec![]);
        new_graph.add_edge(initial_edge);

        //Others Edges
        for edge in self.graph.edge_indices(){
            if let Some((source,target)) = self.graph.edge_endpoints(edge){
                //**** Pretty print of actions for reports ****//
                let label = self.graph.edge_weight(edge).unwrap();
                let new_label = if let Ok(s)= Automaton::simple_parse_action_from_string(&label.to_string()){
                    s.clone()
                }
                else{
                    label.clone().to_string()
                };
                //*********************************//

                let new_edge = GraphVizEdge::new(self.get_node_weight(&source).unwrap().to_string(),None,self.get_node_weight(&target).unwrap().to_string(),None,
                                                 vec![GraphvizEdgeStyleItem::Label(new_label)]);
                new_graph.add_edge(new_edge);
            }

        }

        new_graph
    }
}