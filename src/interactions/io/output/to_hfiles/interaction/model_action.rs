/*
Copyright 2020 Erwan Mahe (github.com/erwanM974)

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/



use crate::interactions::syntax::action::{Action, ActionType};
use crate::interactions::syntax::general_context::GeneralContext;

pub fn action_as_hif_encoding(gen_ctx : &GeneralContext,
                                act : &Action) -> String {

    match act.action_type {
        ActionType::Emission => {

            let lf_name = gen_ctx.get_lf_name(act.lf_id).unwrap();
            let ms_name = gen_ctx.get_ms_name(act.ms_id).unwrap();

            if let Some(g) = act.gate_id{
                return format!("{} -- {} ->| [{}]", &lf_name, &ms_name,&g);
            }
            else{
                return format!("{} -- {} ->|", &lf_name, &ms_name);
            }
        },
        ActionType::Reception => {

            let lf_name = gen_ctx.get_lf_name(act.lf_id).unwrap();
            let ms_name = gen_ctx.get_ms_name(act.ms_id).unwrap();

            if let Some(g) = act.gate_id{
                return format!("{} -> {} [{}]", &ms_name,&lf_name,&g);
            }
            else{
                return format!("{} -> {}", &ms_name,&lf_name);
            }
        }
    }

}

pub fn vp_as_hif_encoding(gen_ctx: &GeneralContext,
                          em_act : &Action, rec_act: &Action) -> String {

    let em_lf_name = gen_ctx.get_lf_name(em_act.lf_id).unwrap();
    let ms_name = gen_ctx.get_ms_name(em_act.ms_id).unwrap();
    let rec_lf_name = gen_ctx.get_lf_name(rec_act.lf_id).unwrap();

    return format!("{} -- {} -> {}", &em_lf_name, &ms_name,&rec_lf_name);

}