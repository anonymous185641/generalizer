


use graph_process_manager_loggers::graphviz::drawer::GraphVizProcessDrawer;
use graph_process_manager_loggers::graphviz::format::GraphVizLoggerNodeFormat;
use graphviz_dot_builder::colors::GraphvizColor;
use graphviz_dot_builder::item::cluster::GraphVizCluster;
use graphviz_dot_builder::item::node::node::GraphVizNode;
use graphviz_dot_builder::item::node::style::{GraphvizNodeStyleItem, GvNodeShape};
use crate::matching::matching_ac::matching_process::matching::conf::{MatchingConfig, MatchingStaticLocalVerdictAnalysisProof};
use crate::matching::matching_ac::matching_process::matching::context::MatchingContext;
use crate::matching::matching_ac::matching_process::matching::loggers::graphviz::drawer::MatchingProcessDrawer;
use crate::matching::matching_ac::matching_process::matching::node::MatchingNodeKind;
use crate::matching::matching_ac::matching_process::matching::param::phase::MatchingParameterization;
use crate::matching::matching_ac::matching_process::matching::step::MatchingStepKind;
use crate::matching::matching_ac::matching_process::matching::verdict::local::MatchingLocalVerdict;

impl GraphVizProcessDrawer<MatchingConfig> for MatchingProcessDrawer {

    fn repr_static_analysis(&self) -> bool {
        false
    }

    fn get_temp_folder(&self) -> &str {
        self.get_temp_folder()
    }

    fn get_verdict_color(&self,
                         local_verdict: &MatchingLocalVerdict) -> GraphvizColor {
        GraphvizColor::black
    }

    fn make_static_analysis_as_gvcluster(&self,
                                         _context: &MatchingContext,
                                         _param : &MatchingParameterization,
                                         parent_state_id: u32,
                                         verdict: &MatchingLocalVerdict,
                                         _data_proof: &MatchingStaticLocalVerdictAnalysisProof) -> GraphVizCluster {
        panic!("should not be called")
    }

    fn make_step_gvnode(&self,
                        context: &MatchingContext,
                        _param : &MatchingParameterization,
                        origin_state_id: u32,
                        target_state_id: u32,
                        step: &MatchingStepKind) -> GraphVizNode {
        let step_name = format!("s_{}_{}", origin_state_id, target_state_id);
        match *step {
            MatchingStepKind::GoToNextPhase => {
                self.make_graphic_logger_string_label("Final configuration".to_string(),step_name)
            },
            MatchingStepKind::Transform(ref transfo) => {
                self.make_graphic_logger_transformation(&transfo.kind,step_name)
            }
        }
    }

    fn make_node_gvitem_as_gvcluster(&self,
                                     context: &MatchingContext,
                                     _parem: &MatchingParameterization,
                                     new_state_id: u32,
                                     new_node: &MatchingNodeKind) -> GraphVizCluster {
        let mut cluster = self.make_graphic_logger_state(new_state_id,&new_node.configuration);
        let colors = vec![
            GraphvizColor::lightskyblue,
            GraphvizColor::lightgoldenrod1,
            GraphvizColor::seagreen1,
            GraphvizColor::lightsalmon
        ];
        match cluster.style.iter().position(|x| match x {
            GraphvizNodeStyleItem::FillColor(_) => true,
            _ => false
        }) {
            Some(idx) => {
                cluster.style.remove(idx);
            },
            _ => {}
        };
        let color = colors.get((new_node.phase as usize) % colors.len()).unwrap();
        cluster.style.push(GraphvizNodeStyleItem::FillColor(color.clone()));
        cluster
    }

    fn make_node_gvitem_as_gvnode(&self,
                                  context: &MatchingContext,
                                  _param: &MatchingParameterization,
                                  new_state_id: u32,
                                  new_node: &MatchingNodeKind) -> GraphVizNode {
        panic!("should not be called")
    }

    fn get_node_format(&self) -> &GraphVizLoggerNodeFormat {
        &GraphVizLoggerNodeFormat::AnchoredCluster
    }

    fn get_anchor_id(&self, id: u32) -> String {
        self.get_anchor_id(id)
    }

    fn get_node_id(&self, id: u32) -> String {
        self.get_node_id(id)
    }

    fn get_verdict_id(&self, id: u32) -> String {
        panic!("should not be called")
    }

    fn get_static_analysis_ids(&self, id: u32) -> (String, String) {
        panic!("should not be called")
    }

}