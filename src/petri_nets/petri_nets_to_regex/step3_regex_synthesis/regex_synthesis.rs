use std::time::Instant;
use graph_process_manager_core::delegate::delegate::GenericProcessDelegate;
use graph_process_manager_core::manager::manager::GenericProcessManager;
use graph_process_manager_core::manager::verdict::AbstractGlobalVerdict;
use crate::anti_unification::generalization_process::generalization::verdict::global::GeneralizationGlobalVerdict;
use crate::petri_nets::petri_net::petri_net::PetriNet;
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting::conf::PetriNetRewritingConfig;
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting::context::PetriNetRewritingContext;
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting::node::PetriNetRewritingNodeKind;
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting::options::PetriNetRewritingOptions;
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting::param::default::DefaultPetriNetRewritingProcess;
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting::param::phase::PetriNetRewritingParameterization;
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting::priorities::PetriNetRewritingPriorities;
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting::step::PetriNetRewritingStepKind;
use crate::petri_nets::petri_net_rewriter::petri_net_rewriting::verdict::global::PetriNetRewritingGlobalVerdict;
use crate::regex_inf::regex::regex::Regex;

impl PetriNet<Regex<String>>{

    pub fn petri_to_regex_synthesis(&self,logger_file_name: &String, logger_folder:&String,draw:bool) -> Vec<PetriNet<Regex<String>>>{
        let petri_to_regex_synthesis_options = PetriNetRewritingOptions::default();
        //let petri_to_regex_synthesis_options =  PetriNetRewritingOptions::options_with_logger(logger_file_name,logger_folder);
        /*
        let petri_to_regex_synthesis_options =
            match draw{
                true =>{
                    PetriNetRewritingOptions::options_with_logger(logger_file_name,logger_folder)
                },
                false =>{
                    PetriNetRewritingOptions::default()
                }
            };

         */


        let petri_to_regex_synthesis_context = PetriNetRewritingContext::new(vec![]);

        let delegate : GenericProcessDelegate<PetriNetRewritingStepKind,PetriNetRewritingNodeKind,PetriNetRewritingPriorities> =
            GenericProcessDelegate::new(
                petri_to_regex_synthesis_options.strategy,
                petri_to_regex_synthesis_options.priorities
            );

        let petri_to_regex_synthesis_param =
            PetriNetRewritingParameterization::from_default(DefaultPetriNetRewritingProcess::RegexSynthesis,
                                                            petri_to_regex_synthesis_options.search_all);


        let mut petri_to_regex_synthesis_manager : GenericProcessManager<PetriNetRewritingConfig> =
            GenericProcessManager::new(
                petri_to_regex_synthesis_context,
                petri_to_regex_synthesis_param,
                delegate,
                petri_to_regex_synthesis_options.filters,
                petri_to_regex_synthesis_options.loggers,
                Some(PetriNetRewritingGlobalVerdict::get_baseline_verdict()),
                true
            );

        let init_petri_net = self.clone();
        let init_node = PetriNetRewritingNodeKind::new(init_petri_net,0);
        //**
        let now = Instant::now();
        let (node_count,global_verdict) = petri_to_regex_synthesis_manager.start_process(init_node);
        let elapsed_time = now.elapsed();

        println!("Petri net rewriting done. Elapsed time: {:?}",elapsed_time);

        global_verdict.final_petri_nets
    }
}
























