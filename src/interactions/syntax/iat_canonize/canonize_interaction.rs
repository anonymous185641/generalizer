use crate::interactions::syntax::general_context::GeneralContext;
use crate::interactions::syntax::iat_canonize::canonize::conf::CanonizationConfig;
use crate::interactions::syntax::iat_canonize::canonize::context::CanonizationContext;
use crate::interactions::syntax::iat_canonize::canonize::node::CanonizationNodeKind;
use crate::interactions::syntax::iat_canonize::canonize::options::HibouCanonizeOptions;
use crate::interactions::syntax::iat_canonize::canonize::param::default::DefaultCanonizationProcess;
use crate::interactions::syntax::iat_canonize::canonize::param::phase::CanonizationParameterization;
use crate::interactions::syntax::iat_canonize::canonize::priorities::CanonizationPriorities;
use crate::interactions::syntax::iat_canonize::canonize::step::CanonizationStepKind;
use crate::interactions::syntax::interaction::Interaction;
use graph_process_manager_core::delegate::delegate::GenericProcessDelegate;
use graph_process_manager_core::manager::manager::GenericProcessManager;


impl Interaction{
    pub fn iat_canonize(&self, gen_ctx: &GeneralContext) -> Interaction {
        let canon_ctx = CanonizationContext::new(gen_ctx.clone());
        let canon_opts : HibouCanonizeOptions = HibouCanonizeOptions::default();

        let delegate : GenericProcessDelegate<CanonizationStepKind,CanonizationNodeKind,CanonizationPriorities> =
            GenericProcessDelegate::new(
                canon_opts.strategy,
                canon_opts.priorities
            );

        let canon_param = CanonizationParameterization::from_default(
            //DefaultCanonizationProcess::FivePhases,
            DefaultCanonizationProcess::Basic,
            canon_opts.search_all
        );

        let mut canon_manager : GenericProcessManager<CanonizationConfig> = GenericProcessManager::new(
            canon_ctx,
            canon_param,
            delegate,
            canon_opts.filters,
            canon_opts.loggers,
            None,
            true
        );

        // ***
        let init_node = CanonizationNodeKind::new(self.clone(),0);
        // ***


        let (_node_count,verdict) = canon_manager.start_process(init_node);

        verdict.canonized_ints[0].clone()
    }
}
