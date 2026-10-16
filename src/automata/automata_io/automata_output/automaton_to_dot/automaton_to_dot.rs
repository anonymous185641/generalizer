use graphviz_dot_builder::edge::edge::GraphVizEdge;
use graphviz_dot_builder::edge::style::GraphvizEdgeStyleItem;
use graphviz_dot_builder::graph::graph::GraphVizDiGraph;
use graphviz_dot_builder::graph::style::{GraphvizGraphStyleItem, GvGraphRankDir};
use graphviz_dot_builder::item::node::node::GraphVizNode;
use graphviz_dot_builder::item::node::style::{GraphvizNodeStyleItem, GvNodeShape};
use graphviz_dot_builder::traits::DotBuildable;
use crate::automata::automaton::automaton::Automaton;

impl Automaton{
    pub fn to_dot(&self)->GraphVizDiGraph{
        let mut new_graph = GraphVizDiGraph::new(vec![GraphvizGraphStyleItem::Rankdir(GvGraphRankDir::LR)]);

        //Starting node
        if self.initial_states.len()!=1{
            panic!("There must be only one initial state for automata");
        }
        let init = GraphVizNode::new("init".to_string(), vec![GraphvizNodeStyleItem::Shape(GvNodeShape::Point)]);
        new_graph.add_node(init.clone());
        //Nodes
        for node in self.graph.node_indices(){
            if self.final_states.contains(&node){
                let new_node = GraphVizNode::new(self.get_node_weight(&node).unwrap(),
                                                 vec![GraphvizNodeStyleItem::Shape(GvNodeShape::DoubleCircle)] );
                //GraphvizNodeStyleItem::Label(node.index().to_string())
                new_graph.add_node(new_node);
            }
            else {
                //  let new_node = GraphVizNode::new(node.index().to_string(), vec![GraphvizNodeStyleItem::Shape(GvNodeShape::Circle)] );
                let new_node = GraphVizNode::new(self.get_node_weight(&node).unwrap(),
                                                 vec![GraphvizNodeStyleItem::Shape(GvNodeShape::Circle)] );
                new_graph.add_node(new_node);
                //GraphvizNodeStyleItem::Label(node.index().to_string()),
            }
        }

        //Initial edge
        let initial_edge = GraphVizEdge::new(init.id, None, self.get_node_weight(&self.initial_states[0]).unwrap(), None, vec![]);
        new_graph.add_edge(initial_edge);

        //Others Edges
        for edge in self.graph.edge_indices(){
            if let Some((source,target)) = self.graph.edge_endpoints(edge){
                //**** Pretty print of actions for reports ****//
                let label = self.graph.edge_weight(edge).unwrap();
                let new_label = if let Ok(s)= Automaton::simple_parse_action_from_string(label){
                    s.clone()
                }
                else{
                    label.clone()
                };
                //*********************************//

                let new_edge = GraphVizEdge::new(self.get_node_weight(&source).unwrap(),None,self.get_node_weight(&target).unwrap(),None,
                                                 vec![GraphvizEdgeStyleItem::Label(new_label)]);
                new_graph.add_edge(new_edge);
            }

        }

        new_graph
    }
}