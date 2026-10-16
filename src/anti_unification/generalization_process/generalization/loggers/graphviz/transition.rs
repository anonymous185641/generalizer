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





use std::collections::{BTreeSet, HashMap, HashSet};
use std::path::{Path, PathBuf};


use graphviz_dot_builder::colors::GraphvizColor;
use graphviz_dot_builder::item::node::node::GraphVizNode;
use graphviz_dot_builder::item::node::style::{GraphvizNodeStyle, GraphvizNodeStyleItem, GvNodeShape};
use image::Rgb;
use image_colored_text::draw::multi_line::MultiLineTextAlignment;
use image_colored_text::ttp::TextToPrint;
use crate::anti_unification::configuration::configuration::Configuration;
use crate::anti_unification::generalization_process::generalization::loggers::graphviz::drawer::GeneralizationProcessDrawer;
use crate::anti_unification::generalization_process::generalization_transformations::transfokind::ConfigurationTransformationKind;
use crate::interactions::io::output::draw_commons::hibou_color_palette::{HCP_BLACK, HCP_STANDARD_PURPLE};
use crate::interactions::io::output::draw_commons::make_image_of_text::new_image_with_colored_text;



impl GeneralizationProcessDrawer {

    pub(crate) fn make_graphic_logger_string_label(&self,
                                            string_label : String,
                                            name : String) -> GraphVizNode {
        let image_file_path : PathBuf = [&self.temp_folder, &format!("{}.png",name)].iter().collect();
        // ***
        Self::draw_string_label(image_file_path.as_path(),string_label);
        // ***
        let mut gv_node_options : GraphvizNodeStyle = Vec::new();
        gv_node_options.push( GraphvizNodeStyleItem::Image( image_file_path.into_os_string().to_str().unwrap().to_string() ) );
        gv_node_options.push(GraphvizNodeStyleItem::Label( "".to_string() ));
        gv_node_options.push(GraphvizNodeStyleItem::FillColor( GraphvizColor::white ));
        gv_node_options.push( GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle) );
        // ***
        GraphVizNode::new(name,gv_node_options)
    }

    pub(crate) fn make_graphic_logger_transformation(&self,
                                              transfo_kind : &ConfigurationTransformationKind,
                                              name : String) -> GraphVizNode {
        let image_file_path : PathBuf = [&self.temp_folder, &format!("{}.png",name)].iter().collect();
        // ***
        Self::draw_transformation(image_file_path.as_path(),transfo_kind);
        // ***
        let mut gv_node_options : GraphvizNodeStyle = Vec::new();
        gv_node_options.push( GraphvizNodeStyleItem::Image( image_file_path.into_os_string().to_str().unwrap().to_string() ) );
        gv_node_options.push(GraphvizNodeStyleItem::Label( "".to_string() ));
        gv_node_options.push(GraphvizNodeStyleItem::FillColor( GraphvizColor::white ));
        gv_node_options.push( GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle) );
        // ***
        GraphVizNode::new(name,gv_node_options)
    }


    pub fn draw_string_label(path : &Path,
                             string_label : String) {
        // ***
        let mut text_lines : Vec<Vec<TextToPrint>> = Vec::new();
        // ***
        {
            let mut ttp = Vec::new();
            // ***
            ttp.push( TextToPrint::new(string_label,Rgb(HCP_BLACK)) );
            text_lines.push( ttp );
        }
        // ***
        new_image_with_colored_text(path,
                                    &MultiLineTextAlignment::Center,
                                    &text_lines);
    }

    pub fn draw_transformation(path : &Path,
                               transfo_kind : &ConfigurationTransformationKind) {
        // ***
        let mut text_lines : Vec<Vec<TextToPrint>> = Vec::new();
        // ***
        {
            let mut ttp = Vec::new();
            // ***
            ttp.push( TextToPrint::new(transfo_kind.to_string(),Rgb(HCP_BLACK)) );
            //ttp.push( TextToPrint::new("@".to_string(),Rgb(HCP_STANDARD_PURPLE)) );
            text_lines.push( ttp );
        }
        // ***
        new_image_with_colored_text(path,
                                    &MultiLineTextAlignment::Center,
                                    &text_lines);
    }



}



