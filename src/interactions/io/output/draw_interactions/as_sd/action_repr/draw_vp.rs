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




use std::collections::HashMap;

use image::{Rgb, RgbImage};
use image_colored_text::draw::single_line::{draw_line_of_colored_text, DrawCoord};
use image_colored_text::ttp::TextToPrint;
use crate::interactions::io::output::draw_commons::font::{get_font, HIBOU_FONT_SCALE};
use crate::interactions::io::output::draw_commons::hibou_color_palette::{HCP_BLACK, HC_MESSAGE};
use crate::interactions::io::output::draw_commons::sd_drawing_conf::VERTICAL_SIZE;
use crate::interactions::io::output::draw_interactions::as_sd::action_repr::common::draw_line_for_message_exchange;
use crate::interactions::io::output::draw_interactions::as_sd::util::arrow_heads::{draw_arrowhead_leftward, draw_arrowhead_rightward};
use crate::interactions::io::output::draw_interactions::as_sd::util::dimensions_tools::get_y_pos_from_yshift;
use crate::interactions::io::output::draw_interactions::as_sd::util::lf_coords::DrawingLifelineCoords;
use crate::interactions::syntax::action::{Action, ActionType};
use crate::interactions::syntax::general_context::GeneralContext;
// **********

pub fn draw_vp( image : &mut RgbImage,
                      gen_ctx: &GeneralContext,
                      act1 : &Action,
                      act2: &Action,
                      lf_x_widths : &HashMap<usize,DrawingLifelineCoords>,
                      yshift : u32) -> [usize;2] {

    if (act1.action_type == act2.action_type) || (act1.ms_id != act2.ms_id) {
        println!("{:?} \n {:?}",act1,act2);
        panic!("Invalid value-passing");
    }

    let em_act;
    let rec_act;

    if act1.action_type == ActionType::Emission{
        em_act = act1;
        rec_act = act2;
    }
    else{
        em_act = act2;
        rec_act = act1;
    }

    // *** Degueulace, à réécrire
    //let mut min_lf_id : usize = em_act.lf_id;
    //let mut max_lf_id : usize = rec_act.lf_id;
    let mut temp_arr = [em_act.lf_id,rec_act.lf_id];
    temp_arr.sort();
    let mut min_lf_id : usize = temp_arr[0];
    let mut max_lf_id : usize = temp_arr[1];
    // ***
    let msg_to_print : Vec<TextToPrint>;
    {
        let msg_label = gen_ctx.get_ms_name(em_act.ms_id).unwrap();
        msg_to_print = vec![TextToPrint::new(msg_label,Rgb(HC_MESSAGE))];
    }
    // ***
    let text_y_pos = get_y_pos_from_yshift(yshift) + VERTICAL_SIZE/2.0;
    let arrow_y_pos = get_y_pos_from_yshift(yshift+2);
    let _msg_to_print_width = TextToPrint::get_text_width(&msg_to_print, &get_font(), &HIBOU_FONT_SCALE);
    // ***
    let (_img_width,_) = image.dimensions();
    // ***
    let origin_lf_id = *(&em_act.lf_id);
    let origin_lf_coords = lf_x_widths.get(&origin_lf_id).unwrap();

    let target_lf_id = rec_act.lf_id;
    let target_lf_coords = lf_x_widths.get(&target_lf_id).unwrap();

    {
        min_lf_id = min_lf_id.min(target_lf_id);
        max_lf_id = max_lf_id.max(target_lf_id);
    }

    if origin_lf_id < target_lf_id {
        draw_arrowhead_rightward(image,target_lf_coords.x_middle, arrow_y_pos,Rgb(HCP_BLACK));
    } else {
        draw_arrowhead_leftward(image,target_lf_coords.x_middle, arrow_y_pos,Rgb(HCP_BLACK));
    }
    draw_line_for_message_exchange(image,target_lf_coords.x_middle,origin_lf_coords.x_middle,arrow_y_pos);
    // ***
    let mut anchor_lf_id : usize = target_lf_id;
    if target_lf_id == origin_lf_id {
        panic!("cannot draw emission then reception on the same lifeline");
    } else if target_lf_id < origin_lf_id {
        let mut lf_id_shift : usize = 1;
        while !lf_x_widths.contains_key(&(origin_lf_id - lf_id_shift)) {
            lf_id_shift = lf_id_shift + 1 ;
        }
        anchor_lf_id = origin_lf_id - lf_id_shift;
    } else if target_lf_id > origin_lf_id {
        let mut lf_id_shift : usize = 1;
        while !lf_x_widths.contains_key(&(origin_lf_id + lf_id_shift)) {
            lf_id_shift = lf_id_shift + 1 ;
        }
        anchor_lf_id = origin_lf_id + lf_id_shift;
    }
    let anchor_lf_coords = lf_x_widths.get(&anchor_lf_id).unwrap();
    let msg_x_middle = (origin_lf_coords.x_middle + anchor_lf_coords.x_middle)/2.0;
    draw_line_of_colored_text(image,
                              &DrawCoord::CenteredAround(msg_x_middle),
                              &DrawCoord::CenteredAround(text_y_pos),
                              &msg_to_print,
                              &get_font(),
                              &HIBOU_FONT_SCALE);
    // ***
    return [min_lf_id,max_lf_id];
}


