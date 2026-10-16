


use graph_process_manager_loggers::graphviz::drawer::GraphVizProcessDrawer;
use graph_process_manager_loggers::graphviz::format::GraphVizLoggerNodeFormat;
use graphviz_dot_builder::colors::GraphvizColor;
use graphviz_dot_builder::item::cluster::GraphVizCluster;
use graphviz_dot_builder::item::node::node::GraphVizNode;
use graphviz_dot_builder::item::node::style::{GraphvizNodeStyleItem, GvNodeShape};
use graphviz_dot_builder::traits::DotBuildable;
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting::conf::{PetriNetRewritingConfig, PetriNetRewritingStaticLocalVerdictAnalysisProof};
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting::context::PetriNetRewritingContext;
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting::loggers::graphviz::drawer::PetriNetRewritingProcessDrawer;
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting::node::PetriNetRewritingNodeKind;
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting::param::phase::PetriNetRewritingParameterization;
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting::step::PetriNetRewritingStepKind;
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting::verdict::local::PetriNetRewritingLocalVerdict;

impl GraphVizProcessDrawer<PetriNetRewritingConfig> for PetriNetRewritingProcessDrawer {

    fn repr_static_analysis(&self) -> bool {
        false
    }

    fn get_temp_folder(&self) -> &str {
        self.get_temp_folder()
    }

    fn get_verdict_color(&self,
                         local_verdict: &PetriNetRewritingLocalVerdict) -> GraphvizColor {
        GraphvizColor::black
    }

    fn make_static_analysis_as_gvcluster(&self,
                                         _context: &PetriNetRewritingContext,
                                         _param : &PetriNetRewritingParameterization,
                                         parent_state_id: u32,
                                         verdict: &PetriNetRewritingLocalVerdict,
                                         _data_proof: &PetriNetRewritingStaticLocalVerdictAnalysisProof) -> GraphVizCluster {
        panic!("should not be called")
    }

    fn make_step_gvnode(&self,
                        context: &PetriNetRewritingContext,
                        _param : &PetriNetRewritingParameterization,
                        origin_state_id: u32,
                        target_state_id: u32,
                        step: &PetriNetRewritingStepKind) -> GraphVizNode {
        let step_name = format!("s_{}_{}", origin_state_id, target_state_id);
        match *step {
            PetriNetRewritingStepKind::GoToNextPhase => {
                self.make_graphic_logger_string_label("Final Petri Net".to_string(),step_name)
            },
            PetriNetRewritingStepKind::Transform(ref transfo) => {
                self.make_graphic_logger_transformation(&transfo.kind,step_name)
            }
        }
    }

    fn make_node_gvitem_as_gvcluster(&self,
                                     context: &PetriNetRewritingContext,
                                     _parem: &PetriNetRewritingParameterization,
                                     new_state_id: u32,
                                     new_node: &PetriNetRewritingNodeKind) -> GraphVizCluster {
        let mut cluster = self.make_graphic_logger_state(new_state_id,&new_node.petri_net);
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
                                  context: &PetriNetRewritingContext,
                                  _param: &PetriNetRewritingParameterization,
                                  new_state_id: u32,
                                  new_node: &PetriNetRewritingNodeKind) -> GraphVizNode {
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