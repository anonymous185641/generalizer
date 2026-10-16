use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use graph_process_manager_core::delegate::delegate::GenericProcessDelegate;
use graph_process_manager_core::manager::manager::GenericProcessManager;
use graph_process_manager_core::manager::verdict::AbstractGlobalVerdict;
use crate::anti_unification::configuration::configuration::Configuration;
use crate::anti_unification::generalization_process::generalization::conf::GeneralizationConfig;
use crate::anti_unification::generalization_process::generalization::context::GeneralizationContext;
use crate::anti_unification::generalization_process::generalization::filter::filter::GeneralizationFilter;
use crate::anti_unification::generalization_process::generalization::node::GeneralizationNodeKind;
use crate::anti_unification::generalization_process::generalization::options::GeneralizeOptions;
use crate::anti_unification::generalization_process::generalization::param::default::DefaultGeneralizationProcess;
use crate::anti_unification::generalization_process::generalization::param::phase::GeneralizationParameterization;
use crate::anti_unification::generalization_process::generalization::priorities::GeneralizationPriorities;
use crate::anti_unification::generalization_process::generalization::step::GeneralizationStepKind;
use crate::anti_unification::generalization_process::generalization::verdict::global::GeneralizationGlobalVerdict;
use crate::anti_unification::generalizer::generalizer::Generalizer;
use crate::terms::term::Term;

impl Generalizer {
    pub fn generalize(t1: &Term,t2: &Term,sc_preserving:bool,variants:bool,loggers_folder: Option<&str>,anti_unif_timout_secs:Option<usize>) -> Vec<Generalizer> {

        //timemout setup
        let filter_timeout = anti_unif_timout_secs.unwrap_or_else(|| 100000);

        //let generalization_options = GeneralizeOptions::options_with_logger();
        let generalization_options = if let Some(folder) = loggers_folder{
            GeneralizeOptions::options_with_logger(folder,variants)
        }else{
            GeneralizeOptions::options_without_loggers(variants,Some(filter_timeout))
        };

        let generalization_ctx = GeneralizationContext::new(vec![]);

        let delegate : GenericProcessDelegate<GeneralizationStepKind,GeneralizationNodeKind,GeneralizationPriorities> =
            GenericProcessDelegate::new(
                generalization_options.strategy,
                generalization_options.priorities
            );

        let generalization_param =
            match sc_preserving {
                true =>{
                    if variants {
                        GeneralizationParameterization::from_default(
                            DefaultGeneralizationProcess::ScPreservingWithVariant,
                            generalization_options.search_all
                        )
                    }
                    else{
                        GeneralizationParameterization::from_default(
                            DefaultGeneralizationProcess::ScPreserving,
                            generalization_options.search_all
                        )
                    }
                },
                false =>{
                    GeneralizationParameterization::from_default(
                        DefaultGeneralizationProcess::Basic,
                        generalization_options.search_all
                    )
                }
            };

        let mut generalization_manager : GenericProcessManager<GeneralizationConfig> =
            match sc_preserving {
                true =>{
                    GenericProcessManager::new(
                        generalization_ctx,
                        generalization_param,
                        delegate,
                        generalization_options.filters,
                        generalization_options.loggers,
                        Some(GeneralizationGlobalVerdict::get_baseline_verdict()),
                        true
                    )
                },
                false =>{
                    GenericProcessManager::new(
                        generalization_ctx,
                        generalization_param,
                        delegate,
                        generalization_options.filters,
                        generalization_options.loggers,
                        None,
                        true
                    )
                }
            };
        // ***
        let init_conf = Configuration::init_conf(t1,t2);
        let init_node = GeneralizationNodeKind::new(init_conf,0);
        // ***

        /*
        let now = Instant::now();
        let (node_count,global_verdict) = generalization_manager.start_process(init_node);
        let elapsed_time = now.elapsed();

         */
        let mut generalizers = vec![];


        let now = Instant::now();
        let (node_count,global_verdict) = generalization_manager.start_process(init_node);
        let elapsed_time = now.elapsed();

        println!("Generalization done. Elapsed time: {:?}\n",elapsed_time);

        for conf in global_verdict.final_configurations.iter() {

            if conf.active.is_empty(){// We only consider configuration with empty active set.
                let gen = conf.to_generaliser();
                generalizers.push(gen);
            }

        }

        generalizers

    }
}
