use graph_process_manager_core::delegate::priorities::GenericProcessPriorities;
use graph_process_manager_core::handler::filter::AbstractFilter;
use graph_process_manager_core::manager::logger::AbstractProcessLogger;
use graph_process_manager_core::queued_steps::queue::strategy::QueueSearchStrategy;
use graph_process_manager_loggers::graphviz::format::GraphVizProcessLoggerLayout;
use graph_process_manager_loggers::graphviz::logger::GenericGraphVizLogger;
use graphviz_dot_builder::traits::GraphVizOutputFormat;
use crate::matching::matching_ac::matching_process::matching::conf::MatchingConfig;
use crate::matching::matching_ac::matching_process::matching::filter::elim::MatchingFilterEliminationKind;
use crate::matching::matching_ac::matching_process::matching::filter::filter::MatchingFilterCriterion;
use crate::matching::matching_ac::matching_process::matching::loggers::graphviz::drawer::MatchingProcessDrawer;
use crate::matching::matching_ac::matching_process::matching::priorities::MatchingPriorities;

pub struct MatchingOptions {
    pub loggers : Vec<Box<dyn AbstractProcessLogger<MatchingConfig>>>,
    pub strategy : QueueSearchStrategy,
    pub filters : Vec<Box<dyn AbstractFilter<MatchingFilterCriterion, MatchingFilterEliminationKind>>>,
    pub priorities : GenericProcessPriorities<MatchingPriorities>,
    pub search_all : bool,
}


impl MatchingOptions {
    fn new(loggers : Vec<Box<dyn AbstractProcessLogger<MatchingConfig>>>,
           strategy : QueueSearchStrategy,
           filters : Vec<Box<dyn AbstractFilter<MatchingFilterCriterion, MatchingFilterEliminationKind>>>,
           priorities : GenericProcessPriorities<MatchingPriorities>,
           search_all : bool) -> MatchingOptions {
        return MatchingOptions {loggers,strategy,filters,priorities,search_all};
    }

    pub fn default() -> MatchingOptions {
        MatchingOptions::new(
            Vec::new(),
            QueueSearchStrategy::BFS,
            vec![],
            GenericProcessPriorities::new(MatchingPriorities::default(), false),
            true
        )
    }

    pub fn options_with_logger() -> MatchingOptions {
        let logger = GenericGraphVizLogger::new(Box::new(MatchingProcessDrawer::new("temp".to_string())),
                                                                                    GraphVizOutputFormat::svg,
                                                                                    GraphVizProcessLoggerLayout::Horizontal,
                                                                                    true,
                                                                                    "Matching_output".to_string(),
                                                                                    "derivation".to_string());
        MatchingOptions::new(
            vec![Box::new(logger)],
            QueueSearchStrategy::DFS,
            vec![],
            GenericProcessPriorities::new(MatchingPriorities::default(), false),
            true
        )
    }

}
