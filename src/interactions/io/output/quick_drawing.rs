use std::path::{Path};
use crate::interactions::io::output::draw_interactions::interface::{draw_interaction, InteractionGraphicalRepresentation};
use crate::interactions::io::output::to_hfiles::interaction::to_hif::interaction_to_hif;
use crate::interactions::syntax::general_context::GeneralContext;
use crate::interactions::syntax::interaction::Interaction;


pub fn draw_model(gen_ctx: &GeneralContext, model_name: &str, parent_dir: &str, int: &Interaction ){
    draw_interaction(&gen_ctx, int, &InteractionGraphicalRepresentation::AsSequenceDiagram, &"temp".to_string(), &parent_dir.to_string(), &model_name.to_string());
    draw_interaction(&gen_ctx, int, &InteractionGraphicalRepresentation::AsTerm, &"temp".to_string(), &parent_dir.to_string(), &format!("{}_tree",model_name));

    let path_str = format!("{}/{}.hif",parent_dir,model_name);
    interaction_to_hif(&Path::new(&path_str),gen_ctx,int);
}
pub fn draw_model_without_hif(gen_ctx: &GeneralContext, model_name: &str, parent_dir: &str, int: &Interaction ){
    draw_interaction(&gen_ctx, int, &InteractionGraphicalRepresentation::AsSequenceDiagram, &"temp".to_string(), &parent_dir.to_string(), &model_name.to_string());
    draw_interaction(&gen_ctx, int, &InteractionGraphicalRepresentation::AsTerm, &"temp".to_string(), &parent_dir.to_string(), &format!("{}_tree",model_name));
}