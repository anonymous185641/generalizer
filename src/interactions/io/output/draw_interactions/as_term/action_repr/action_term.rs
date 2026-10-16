
use crate::interactions::io::textual_convention::{SYNTAX_EMISSION, SYNTAX_RECEPTION};
use crate::interactions::syntax::action::{Action, ActionType};
use crate::interactions::syntax::general_context::GeneralContext;


pub fn action_as_gv_label(gen_ctx : &GeneralContext,
                            act : &Action) -> String {
    // ***
    let ms_name = gen_ctx.get_ms_name(act.ms_id).unwrap();
    let lf_name = gen_ctx.get_lf_name(act.lf_id).unwrap();
    // ***

    // ***
    let symb : &'static str;
    match act.action_type {
        ActionType::Emission => {
            symb = SYNTAX_EMISSION;
        },
        ActionType::Reception => {
            symb = SYNTAX_RECEPTION;
        }
    }
    // ***
    if let Some(gt_id) = act.gate_id{
        return format!("{}{}{}[{}]", &lf_name, symb, &ms_name, gt_id);
    }

    return format!("{}{}{}", &lf_name, symb, &ms_name);


}