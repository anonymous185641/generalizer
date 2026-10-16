use graph_process_manager_core::delegate::priorities::GenericProcessPriorities;
use graph_process_manager_core::handler::filter::AbstractFilter;
use graph_process_manager_core::manager::logger::AbstractProcessLogger;
use graph_process_manager_core::queued_steps::queue::strategy::QueueSearchStrategy;
use graph_process_manager_loggers::graphviz::format::GraphVizProcessLoggerLayout;
use graph_process_manager_loggers::graphviz::logger::GenericGraphVizLogger;
use graphviz_dot_builder::traits::GraphVizOutputFormat;
use crate::anti_unification::generalization_process::generalization::conf::GeneralizationConfig;
use crate::anti_unification::generalization_process::generalization::filter::elim::GeneralizationFilterEliminationKind;
use crate::anti_unification::generalization_process::generalization::filter::filter::{GeneralizationFilter, GeneralizationFilterCriterion};
use crate::anti_unification::generalization_process::generalization::loggers::graphviz::drawer::GeneralizationProcessDrawer;
use crate::anti_unification::generalization_process::generalization::priorities::GeneralizationPriorities;



pub struct GeneralizeOptions {
    pub loggers : Vec<Box<dyn AbstractProcessLogger<GeneralizationConfig>>>,
    pub strategy : QueueSearchStrategy,
    pub filters : Vec<Box<dyn AbstractFilter<GeneralizationFilterCriterion, GeneralizationFilterEliminationKind>>>,
    //pub filters : Vec<Box<GeneralizationFilter>>,
    pub priorities : GenericProcessPriorities<GeneralizationPriorities>,
    pub search_all : bool,
}


impl GeneralizeOptions {
    fn new(loggers : Vec<Box<dyn AbstractProcessLogger<GeneralizationConfig>>>,
           strategy : QueueSearchStrategy,
           filters : Vec<Box<dyn AbstractFilter<GeneralizationFilterCriterion, GeneralizationFilterEliminationKind>>>,
           //filters : Vec<Box<GeneralizationFilter>>,
           priorities : GenericProcessPriorities<GeneralizationPriorities>,
           search_all : bool) -> GeneralizeOptions {
        return GeneralizeOptions {loggers,strategy,filters,priorities,search_all};
    }

    pub fn default() -> GeneralizeOptions {
        GeneralizeOptions::new(
            Vec::new(),
            QueueSearchStrategy::DFS,
            vec![],
            GenericProcessPriorities::new(GeneralizationPriorities::default(), false),
            true
        )
    }

    pub fn options_without_loggers(with_variants:bool,timeout_secs:Option<usize>) -> GeneralizeOptions {
        
        if with_variants{
            GeneralizeOptions::new(
                Vec::new(),
                QueueSearchStrategy::DFS,
                vec![Box::new(GeneralizationFilter::Timeout((std::time::Instant::now(),timeout_secs.unwrap())))],
                GenericProcessPriorities::new(GeneralizationPriorities::default(), false),
                true
            )
        }
        else{
            GeneralizeOptions::new(
                Vec::new(),
                QueueSearchStrategy::DFS,
                vec![Box::new(GeneralizationFilter::Timeout((std::time::Instant::now(),timeout_secs.unwrap())))],
                GenericProcessPriorities::new(GeneralizationPriorities::default(), false),
                true
            )
        }

    }

    pub fn options_with_logger(logger_folder: &str,with_variants:bool) -> GeneralizeOptions {
        let logger_folder_str = if !logger_folder.is_empty() {
            format!("{}/Anti_unification_output", logger_folder)
        }else{
            "Anti_unification_output".to_string()
        };
        
        let logger = GenericGraphVizLogger::new(Box::new(GeneralizationProcessDrawer::new("temp".to_string())),
                                                                                    GraphVizOutputFormat::png,
                                                                                    GraphVizProcessLoggerLayout::Horizontal,
                                                                                    true,
                                                                                    logger_folder_str,
                                                                                    "derivation".to_string());


        if with_variants{
            GeneralizeOptions::new(
                vec![Box::new(logger)],
                QueueSearchStrategy::DFS,
                vec![],
                GenericProcessPriorities::new(GeneralizationPriorities::default(), false),
                true
            )
        }
        else{
            GeneralizeOptions::new(
                vec![Box::new(logger)],
                QueueSearchStrategy::DFS,
                vec![],
                GenericProcessPriorities::new(GeneralizationPriorities::default(), false),
                true
            )
        }

    }

}
