use graph_process_manager_core::delegate::priorities::GenericProcessPriorities;
use graph_process_manager_core::handler::filter::AbstractFilter;
use graph_process_manager_core::manager::logger::AbstractProcessLogger;
use graph_process_manager_core::queued_steps::queue::strategy::QueueSearchStrategy;
use graph_process_manager_loggers::graphviz::format::GraphVizProcessLoggerLayout;
use graph_process_manager_loggers::graphviz::logger::GenericGraphVizLogger;
use graphviz_dot_builder::traits::GraphVizOutputFormat;
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting::conf::PetriNetRewritingConfig;
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting::filter::elim::PetriNetRewritingFilterEliminationKind;
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting::filter::filter::PetriNetRewritingFilterCriterion;
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting::loggers::graphviz::drawer::PetriNetRewritingProcessDrawer;
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting::priorities::PetriNetRewritingPriorities;

pub struct PetriNetRewritingOptions {
    pub loggers : Vec<Box<dyn AbstractProcessLogger<PetriNetRewritingConfig>>>,
    pub strategy : QueueSearchStrategy,
    pub filters : Vec<Box<dyn AbstractFilter<PetriNetRewritingFilterCriterion, PetriNetRewritingFilterEliminationKind>>>,
    pub priorities : GenericProcessPriorities<PetriNetRewritingPriorities>,
    pub search_all : bool,
}


impl PetriNetRewritingOptions {
    fn new(loggers : Vec<Box<dyn AbstractProcessLogger<PetriNetRewritingConfig>>>,
           strategy : QueueSearchStrategy,
           filters : Vec<Box<dyn AbstractFilter<PetriNetRewritingFilterCriterion, PetriNetRewritingFilterEliminationKind>>>,
           priorities : GenericProcessPriorities<PetriNetRewritingPriorities>,
           search_all : bool) -> PetriNetRewritingOptions {
        return PetriNetRewritingOptions {loggers,strategy,filters,priorities,search_all};
    }

    pub fn default() -> PetriNetRewritingOptions {
        PetriNetRewritingOptions::new(
            Vec::new(),
            QueueSearchStrategy::DFS,
            vec![],
            GenericProcessPriorities::new(PetriNetRewritingPriorities::default(), false),
            true
        )
    }

    pub fn options_with_logger(logger_file_name: &String, logger_folder:&String) -> PetriNetRewritingOptions {
        let logger = GenericGraphVizLogger::new(Box::new(PetriNetRewritingProcessDrawer::new("temp".to_string())),
                                                GraphVizOutputFormat::svg,
                                                GraphVizProcessLoggerLayout::Horizontal,
                                                true,
                                                logger_folder.clone(),
                                                logger_file_name.clone());
        PetriNetRewritingOptions::new(
            vec![Box::new(logger)],
            QueueSearchStrategy::DFS,
            vec![],
            GenericProcessPriorities::new(PetriNetRewritingPriorities::default(), false),
            true
        )
    }

}
