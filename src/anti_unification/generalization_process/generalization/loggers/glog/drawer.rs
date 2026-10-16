


use graph_process_manager_loggers::graphviz::drawer::GraphVizProcessDrawer;
use graph_process_manager_loggers::graphviz::format::GraphVizLoggerNodeFormat;
use graphviz_dot_builder::colors::GraphvizColor;
use graphviz_dot_builder::item::cluster::GraphVizCluster;
use graphviz_dot_builder::item::node::node::GraphVizNode;
use graphviz_dot_builder::item::node::style::{GraphvizNodeStyleItem, GvNodeShape};
use graphviz_dot_builder::traits::DotBuildable;
use crate::anti_unification::generalization_process::generalization::conf::{GeneralizationConfig, GeneralizationStaticLocalVerdictAnalysisProof};
use crate::anti_unification::generalization_process::generalization::context::GeneralizationContext;
use crate::anti_unification::generalization_process::generalization::loggers::graphviz::drawer::GeneralizationProcessDrawer;
use crate::anti_unification::generalization_process::generalization::node::GeneralizationNodeKind;
use crate::anti_unification::generalization_process::generalization::param::phase::GeneralizationParameterization;
use crate::anti_unification::generalization_process::generalization::step::GeneralizationStepKind;
use crate::anti_unification::generalization_process::generalization::verdict::local::GeneralizationLocalVerdict;


impl GraphVizProcessDrawer<GeneralizationConfig> for GeneralizationProcessDrawer {

    fn repr_static_analysis(&self) -> bool {
        false
    }

    fn get_temp_folder(&self) -> &str {
        self.get_temp_folder()
    }

    fn get_verdict_color(&self,
                         local_verdict: &GeneralizationLocalVerdict) -> GraphvizColor {
        GraphvizColor::black
    }

    fn make_static_analysis_as_gvcluster(&self,
                                         _context: &GeneralizationContext,
                                         _param : &GeneralizationParameterization,
                                         parent_state_id: u32,
                                         verdict: &GeneralizationLocalVerdict,
                                         _data_proof: &GeneralizationStaticLocalVerdictAnalysisProof) -> GraphVizCluster {
        panic!("should not be called")
    }

    fn make_step_gvnode(&self,
                        context: &GeneralizationContext,
                        _param : &GeneralizationParameterization,
                        origin_state_id: u32,
                        target_state_id: u32,
                        step: &GeneralizationStepKind) -> GraphVizNode {
        let step_name = format!("s_{}_{}", origin_state_id, target_state_id);
        match *step {
            GeneralizationStepKind::GoToNextPhase => {
                self.make_graphic_logger_string_label("Final configuration".to_string(),step_name)
            },
            GeneralizationStepKind::Transform(ref transfo) => {
                self.make_graphic_logger_transformation(&transfo.kind,step_name)
            }
        }
    }

    fn make_node_gvitem_as_gvcluster(&self,
                                     context: &GeneralizationContext,
                                     _parem: &GeneralizationParameterization,
                                     new_state_id: u32,
                                     new_node: &GeneralizationNodeKind) -> GraphVizCluster {
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
                                  context: &GeneralizationContext,
                                  _param: &GeneralizationParameterization,
                                  new_state_id: u32,
                                  new_node: &GeneralizationNodeKind) -> GraphVizNode {
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