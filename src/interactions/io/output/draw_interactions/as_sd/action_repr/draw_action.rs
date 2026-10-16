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
use crate::interactions::io::output::draw_commons::generate_color::generate_color;
use crate::interactions::io::output::draw_commons::hibou_color_palette::{HCP_BLACK, HC_MESSAGE};
use crate::interactions::io::output::draw_commons::sd_drawing_conf::VERTICAL_SIZE;
use crate::interactions::io::output::draw_interactions::as_sd::action_repr::common::draw_line_for_message_exchange;
use crate::interactions::io::output::draw_interactions::as_sd::util::arrow_heads::draw_arrowhead_rightward;
use crate::interactions::io::output::draw_interactions::as_sd::util::dimensions_tools::get_y_pos_from_yshift;
use crate::interactions::io::output::draw_interactions::as_sd::util::lf_coords::DrawingLifelineCoords;
use crate::interactions::syntax::action::{Action, ActionType};
use crate::interactions::syntax::general_context::GeneralContext;


// **********

pub fn draw_action( image : &mut RgbImage,
                      gen_ctx: &GeneralContext,
                      act : &Action,
                      lf_x_widths : &HashMap<usize,DrawingLifelineCoords>,
                      yshift : u32) -> [usize;2] {



    match act.action_type {
        ActionType::Emission => {

            // ***
            let min_lf_id : usize = act.lf_id;
            let max_lf_id : usize = act.lf_id;
            // ***
            let msg_to_print : Vec<TextToPrint>;
            {
                let msg_label = gen_ctx.get_ms_name(act.ms_id).unwrap();
                msg_to_print = vec![TextToPrint::new(msg_label,Rgb(HC_MESSAGE))];
            }
            // ***
            let text_y_pos = get_y_pos_from_yshift(yshift) + VERTICAL_SIZE/2.0;
            let arrow_y_pos = get_y_pos_from_yshift(yshift+2);
            let _msg_to_print_width = TextToPrint::get_text_width(&msg_to_print, &get_font(), &HIBOU_FONT_SCALE);
            // ***
            let (_img_width,_) = image.dimensions();
            // ***
            let main_lf_coords = lf_x_widths.get(&act.lf_id).unwrap();
            // ***

            let msg_x_left = main_lf_coords.x_middle;
            let msg_x_right= msg_x_left +(main_lf_coords.x_span_inner/2.0);
            draw_arrowhead_rightward(image,msg_x_right,arrow_y_pos,Rgb(HCP_BLACK));
            draw_line_for_message_exchange(image,msg_x_left,msg_x_right,arrow_y_pos);
            let msg_x_middle = (msg_x_left + msg_x_right)/2.0;
            draw_line_of_colored_text(image,
                                      &DrawCoord::CenteredAround(msg_x_middle),
                                      &DrawCoord::CenteredAround(text_y_pos),
                                      &msg_to_print,
                                      &get_font(),
                                      &HIBOU_FONT_SCALE);

            /***   ****/
            // let color: Rgb<u8> = Rgb([((gate.gateway_id-1)*100/255) as u8, 100 as u8, 150 as u8]);
            if act.gate_id != None{
                //let color = generate_color(act.gateway_id.unwrap()*11100011110011);
                let color = generate_color(act.gate_id.unwrap());
                for x in (msg_x_right as u32)..(msg_x_right as u32)+(20 as u32){
                    for y in  (arrow_y_pos as u32)-10.. (arrow_y_pos as u32)+(10 as u32){
                        image.put_pixel(x as u32, y as u32, color);
                    }
                }
            }

            /***   ****/

            return [min_lf_id,max_lf_id];
        },
        ActionType::Reception =>{
            // ***
            let mut min_lf_id : usize = gen_ctx.get_lf_num();
            let mut max_lf_id : usize = 0;
            // ***
            let msg_to_print : Vec<TextToPrint>;
            {
                let msg_label = gen_ctx.get_ms_name(act.ms_id).unwrap();
                msg_to_print = vec![TextToPrint::new(msg_label,Rgb(HC_MESSAGE))];
            }
            // ***
            let text_y_pos = get_y_pos_from_yshift(yshift) + VERTICAL_SIZE/2.0;
            let arrow_y_pos = get_y_pos_from_yshift(yshift+2);
            let _msg_to_print_width = TextToPrint::get_text_width(&msg_to_print,
                                                                  &get_font(),
                                                                  &HIBOU_FONT_SCALE);
            // ***
            let (_img_width,_) = image.dimensions();
            // ***

            let rcv_lf_id = act.lf_id;

            {
                min_lf_id = min_lf_id.min(rcv_lf_id);
                max_lf_id = max_lf_id.max(rcv_lf_id);
            }
            let tar_lf_coords = lf_x_widths.get(&rcv_lf_id).unwrap();
            // ***
            let tar_x_right = tar_lf_coords.x_middle;
            let tar_x_left= tar_x_right - (tar_lf_coords.x_span_inner/2.0);
            draw_arrowhead_rightward(image, tar_x_right, arrow_y_pos,Rgb(HCP_BLACK));
            draw_line_for_message_exchange(image,tar_x_left,tar_x_right,arrow_y_pos);
            let msg_x_middle = (tar_x_left + tar_x_right)/2.0;
            draw_line_of_colored_text(image,
                                      &DrawCoord::CenteredAround(msg_x_middle),
                                      &DrawCoord::CenteredAround(text_y_pos),
                                      &msg_to_print,
                                      &get_font(),
                                      &HIBOU_FONT_SCALE);


            /***   ****/
            //let color: Rgb<u8> = Rgb([((gate.gateway_id-1)*50/255) as u8, 100 as u8, 150 as u8]);
            if act.gate_id != None{
                //let color = generate_color(act.gateway_id.unwrap()*11100011110011);
                let color = generate_color(act.gate_id.unwrap());

                for x in (tar_x_left as u32)-20..(tar_x_left as u32){
                    for y in  (arrow_y_pos as u32)-10.. (arrow_y_pos as u32)+(10 as u32){
                        image.put_pixel(x as u32, y as u32, color);
                    }
                }
            }

            /***   ****/

            return [min_lf_id,max_lf_id];
        }


    }

    // ***

}


