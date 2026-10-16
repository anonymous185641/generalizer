
use std::path::PathBuf;
use graphviz_dot_builder::colors::GraphvizColor;
use graphviz_dot_builder::item::cluster::GraphVizCluster;
use graphviz_dot_builder::item::node::node::GraphVizNode;
use graphviz_dot_builder::item::node::style::{GraphvizNodeStyle, GraphvizNodeStyleItem, GvNodeShape, GvNodeStyleKind};
use graphviz_dot_builder::traits::DotBuildable;
use crate::anti_unification::configuration::configuration::Configuration;
use crate::matching::matching_ac::matching_configuration::matching_configuration::MatchingConfiguration;
use crate::matching::matching_ac::matching_process::matching::loggers::graphviz::drawer::MatchingProcessDrawer;

impl MatchingProcessDrawer {

    pub(crate) fn make_anchor_node(&self, state_id : u32) -> GraphVizNode {
        GraphVizNode::new(self.get_anchor_id(state_id),
                          vec![GraphvizNodeStyleItem::Label("".to_string()),
                               GraphvizNodeStyleItem::Style(vec![GvNodeStyleKind::Invis]),
                               GraphvizNodeStyleItem::Peripheries(1),
                               GraphvizNodeStyleItem::Height(0),GraphvizNodeStyleItem::Width(0)
                          ])
    }

    pub(crate) fn make_graphic_logger_state(&self,
                                     state_id : u32,
                                     config : &MatchingConfiguration)
                                     -> GraphVizCluster {
        // ***
        let cluster_gv_options = vec![ GraphvizNodeStyleItem::FillColor( GraphvizColor::lightgrey ),
                                       GraphvizNodeStyleItem::Label( "".to_string() )];
        // ***
        let mut cluster = GraphVizCluster::new( self.get_node_id(state_id),
                                                cluster_gv_options,
                                                vec![],
                                                vec![]);

       

        // ***
        let node = self.make_graphic_logger_conf(config, format!("isd{}", state_id));
        cluster.add_node(node);
        // ***

        let anchor = self.make_anchor_node(state_id);
        cluster.add_node(anchor);


        return cluster;
    }



    pub(crate) fn make_graphic_logger_conf(&self,
                                         config : &MatchingConfiguration,
                                         name : String) -> GraphVizNode {
        let mut node_gv_options : GraphvizNodeStyle = Vec::new();

        //node_gv_options.push(GraphvizNodeStyleItem::Label( format!("{}",config) ));
        config.draw(&name,&self.temp_folder);
        let int_image_file_path : PathBuf = [&self.temp_folder, &format!("{}.png",name)].iter().collect();
        node_gv_options.push( GraphvizNodeStyleItem::Image( int_image_file_path.into_os_string().to_str().unwrap().to_string() ) );
        node_gv_options.push(GraphvizNodeStyleItem::Label( "".to_string() ));


        node_gv_options.push(GraphvizNodeStyleItem::FillColor( GraphvizColor::white ));
        node_gv_options.push( GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle) );
        // ***
        GraphVizNode::new(name, node_gv_options)
    }

}