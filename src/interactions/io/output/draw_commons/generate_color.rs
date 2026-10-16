use image::Rgb;
use rand::prelude::StdRng;
use rand::{Rng, SeedableRng};
use crate::interactions::io::output::draw_commons::hibou_color_palette::{HCP_LIGHT_BLUE, HCP_LIGHT_CYAN, HCP_LIGHT_GRAY, HCP_LIGHT_GREEN, HCP_LIGHT_ORANGE, HCP_LIGHT_PURPLE, HCP_LIGHT_RED, HCP_LIGHT_YELLOW};

pub fn generate_color(seed: usize) -> Rgb<u8>{
    if seed == 1{
        Rgb(HCP_LIGHT_RED)
    }
    else if seed == 2{
        Rgb(HCP_LIGHT_BLUE)
    }
    else if seed == 3{
        Rgb(HCP_LIGHT_ORANGE)
    }
    else if seed == 4{
        Rgb(HCP_LIGHT_CYAN)

    }
    else if seed == 5{
        Rgb(HCP_LIGHT_GRAY)
    }
    else if seed == 6{
        Rgb(HCP_LIGHT_YELLOW)
    }
    else if seed == 7{
        Rgb(HCP_LIGHT_GREEN)
    }
    else {
        //TO DO: Check that the color generated is different from the ones used above
        let mut rng = StdRng::seed_from_u64((seed*11100) as u64);
        Rgb([rng.random(), rng.random(), rng.random()])
    }
}
