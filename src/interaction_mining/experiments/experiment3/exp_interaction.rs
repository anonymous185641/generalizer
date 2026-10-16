use crate::interactions::syntax::general_context::GeneralContext;
use crate::interactions::syntax::interaction::Interaction;
use crate::regex_inf::regex::regex::Regex;
use crate::regex_inf::sample::sample::Sample;

pub struct ExpInteraction {
    pub interaction: Interaction,
    pub gen_ctx: GeneralContext,
    pub regex: Regex<String>,
    pub sample: Sample,
    pub interaction_semantic_size: usize,
    pub interaction_data_cost: f64,
    pub interaction_ccardinality: usize,
}

impl ExpInteraction {
    
}