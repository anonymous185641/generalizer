use std::time::Instant;
use graph_process_manager_core::delegate::delegate::GenericProcessDelegate;
use graph_process_manager_core::manager::manager::GenericProcessManager;
use graph_process_manager_core::manager::verdict::AbstractGlobalVerdict;
use crate::matching::matching_ac::matching_configuration::matching_configuration::MatchingConfiguration;
use crate::matching::matching_ac::matching_process::matching::conf::MatchingConfig;
use crate::matching::matching_ac::matching_process::matching::context::MatchingContext;
use crate::matching::matching_ac::matching_process::matching::node::MatchingNodeKind;
use crate::matching::matching_ac::matching_process::matching::options::MatchingOptions;
use crate::matching::matching_ac::matching_process::matching::param::default::DefaultMatchingProcess;
use crate::matching::matching_ac::matching_process::matching::param::phase::MatchingParameterization;
use crate::matching::matching_ac::matching_process::matching::priorities::MatchingPriorities;
use crate::matching::matching_ac::matching_process::matching::step::MatchingStepKind;
use crate::matching::matching_ac::matching_process::matching::verdict::global::MatchingGlobalVerdict;
use crate::terms::substitution::substitution::Substitution;
use crate::terms::term::Term;

pub fn match_ac(t1: &Term, t2: &Term,all_matchers:bool) -> Vec<Substitution>{
    //let matching_options = MatchingOptions::options_with_logger();
    let matching_options = MatchingOptions::default();

    let matching_ctx = MatchingContext::new(vec![]);

    let delegate : GenericProcessDelegate<MatchingStepKind,MatchingNodeKind,MatchingPriorities> =
        GenericProcessDelegate::new(
            matching_options.strategy,
            matching_options.priorities
        );

    let matching_param = MatchingParameterization::from_default(
        DefaultMatchingProcess::Basic,
        matching_options.search_all
        );

    let mut matching_manager: GenericProcessManager<MatchingConfig> =
        match all_matchers {
            false =>{
                GenericProcessManager::new(
                    matching_ctx,
                    matching_param,
                    delegate,
                    matching_options.filters,
                    matching_options.loggers,
                    Some(MatchingGlobalVerdict::get_baseline_verdict()),
                    true
                )
            },
            true =>{
                GenericProcessManager::new(
                    matching_ctx,
                    matching_param,
                    delegate,
                    matching_options.filters,
                    matching_options.loggers,
                    None,
                    true
                )
            }
        };
    // ***
    let init_conf = MatchingConfiguration::init_conf(t1,t2);
    let init_node = MatchingNodeKind::new(init_conf,0);
    // ***

    let now = Instant::now();
    let (node_count,global_verdict) = matching_manager.start_process(init_node);
    let elapsed_time = now.elapsed();

    println!("Elapsed time: {:?}",elapsed_time);
    let mut matchers = vec![];
    for conf in global_verdict.final_configurations.iter() {
       // println!("{:?}",conf);
        if let Ok(subs) = conf.extract_substitution(){
            matchers.push(subs);
        }

    }

    matchers
}

















