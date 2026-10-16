use std::time::Instant;
use graph_process_manager_core::delegate::delegate::GenericProcessDelegate;
use graph_process_manager_core::manager::manager::GenericProcessManager;
use crate::petri_nets::petri_net::petri_net::PetriNet;
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting::conf::PetriNetRewritingConfig;
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting::context::PetriNetRewritingContext;
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting::node::PetriNetRewritingNodeKind;
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting::options::PetriNetRewritingOptions;
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting::param::default::DefaultPetriNetRewritingProcess;
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting::param::phase::PetriNetRewritingParameterization;
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting::priorities::PetriNetRewritingPriorities;
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting::step::PetriNetRewritingStepKind;
use crate::regex_inf::regex::regex::Regex;

impl PetriNet<Regex<String>>{
    pub fn alt_int_separation(&self,logger_file_name: &String, logger_folder:&String) -> Vec<PetriNet<Regex<String>>>{

        let alt_int_separation_options = PetriNetRewritingOptions::options_with_logger(logger_file_name,logger_folder);

        let alt_int_separation_context = PetriNetRewritingContext::new(vec![]);

        let delegate : GenericProcessDelegate<PetriNetRewritingStepKind,PetriNetRewritingNodeKind,PetriNetRewritingPriorities> =
            GenericProcessDelegate::new(
                alt_int_separation_options.strategy,
                alt_int_separation_options.priorities
            );

        let alt_int_separation_param =
            PetriNetRewritingParameterization::from_default(DefaultPetriNetRewritingProcess::AltIntSeparation,
                                                            alt_int_separation_options.search_all);


        let mut alt_int_separation_manager : GenericProcessManager<PetriNetRewritingConfig> =
        GenericProcessManager::new(
          alt_int_separation_context,
          alt_int_separation_param,
          delegate,
          alt_int_separation_options.filters,
          alt_int_separation_options.loggers,
          None,
          true
        );

        let init_petri_net = self.clone();
        let init_node = PetriNetRewritingNodeKind::new(init_petri_net,0);
        //**
        let now = Instant::now();
        let (node_count,global_verdict) = alt_int_separation_manager.start_process(init_node);
        let elapsed_time = now.elapsed();

        println!("Petri net rewriting done. Elapsed time: {:?}",elapsed_time);

       global_verdict.final_petri_nets
    }
}
























