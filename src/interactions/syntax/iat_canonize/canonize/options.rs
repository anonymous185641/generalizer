use graph_process_manager_core::delegate::priorities::GenericProcessPriorities;
use graph_process_manager_core::handler::filter::AbstractFilter;
use graph_process_manager_core::manager::logger::AbstractProcessLogger;
use graph_process_manager_core::queued_steps::queue::strategy::QueueSearchStrategy;
use crate::interactions::syntax::iat_canonize::canonize::conf::CanonizationConfig;
use crate::interactions::syntax::iat_canonize::canonize::filter::elim::CanonizationFilterEliminationKind;
use crate::interactions::syntax::iat_canonize::canonize::filter::filter::CanonizationFilterCriterion;
use crate::interactions::syntax::iat_canonize::canonize::priorities::CanonizationPriorities;


pub struct HibouCanonizeOptions {
    pub loggers : Vec<Box<dyn AbstractProcessLogger<CanonizationConfig>>>,
    pub strategy : QueueSearchStrategy,
    pub filters : Vec<Box<dyn AbstractFilter<CanonizationFilterCriterion,CanonizationFilterEliminationKind>>>,
    pub priorities : GenericProcessPriorities<CanonizationPriorities>,
    pub search_all : bool,
}


impl HibouCanonizeOptions {
    fn new(loggers : Vec<Box<dyn AbstractProcessLogger<CanonizationConfig>>>,
           strategy : QueueSearchStrategy,
           filters : Vec<Box<dyn AbstractFilter<CanonizationFilterCriterion,CanonizationFilterEliminationKind>>>,
           priorities : GenericProcessPriorities<CanonizationPriorities>,
           search_all : bool) -> HibouCanonizeOptions {
        return HibouCanonizeOptions{loggers,strategy,filters,priorities,search_all};
    }

    pub fn default() -> HibouCanonizeOptions {
        HibouCanonizeOptions::new(
            Vec::new(),
            QueueSearchStrategy::BFS,
            vec![],
            GenericProcessPriorities::new(CanonizationPriorities::default(),false),
            false
        )
    }

}
