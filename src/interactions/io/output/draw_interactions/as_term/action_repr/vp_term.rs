
use crate::interactions::io::textual_convention::{SYNTAX_EMISSION, SYNTAX_RECEPTION};
use crate::interactions::syntax::action::{Action, ActionType};
use crate::interactions::syntax::general_context::GeneralContext;


pub fn vp_as_gv_label(gen_ctx : &GeneralContext,
                          act1 : &Action,act2 : &Action) -> String {

    //We assume here that act1 and act2 are complementary actions
    // ***
    let ms_name = gen_ctx.get_ms_name(act1.ms_id).unwrap();
    let lf1_name = gen_ctx.get_lf_name(act1.lf_id).unwrap();
    let lf2_name = gen_ctx.get_lf_name(act2.lf_id).unwrap();

    match act1.action_type {
        ActionType::Emission => {
            return format!("vp ({}{}{}{}{})", &lf1_name, SYNTAX_EMISSION, &ms_name, SYNTAX_RECEPTION, lf2_name );
        },
        ActionType::Reception => {
            return format!("vp({}{}{}{}{})", &lf2_name, SYNTAX_EMISSION, &ms_name, SYNTAX_RECEPTION, lf1_name );
        }
    }
}

