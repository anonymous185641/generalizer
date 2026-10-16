use std::fmt::Display;
use crate::anti_unification::configuration::aut::AUT;
use crate::anti_unification::error::ConfigurationError;
use crate::anti_unification::generalizer::generalizer::Generalizer;
use crate::terms::substitution::substitution::Substitution;
use crate::terms::substitution::variable::Variable;
use crate::terms::term::Term;
use image::{Rgb, RgbImage};
use imageproc::drawing::{draw_line_segment_mut, draw_text_mut};
use ab_glyph::{FontArc, PxScale};
use rusttype::Scale;
use crate::interactions::io::output::draw_commons::font::get_font;

#[derive(Clone, PartialEq, Debug, Eq,Hash,Ord,PartialOrd)]
pub struct Configuration {
    pub active : Vec<AUT>,
    pub store : Vec<AUT>,
    pub sub: Vec<Substitution>,
    pub x0 : Variable,
}


impl Configuration {
    pub fn new(active: Vec<AUT>, store: Vec<AUT>, sub: Vec<Substitution>,x0: Variable) -> Self {
        Self { active , store, sub,x0}
    }

    pub fn init_conf(t1: &Term, t2: &Term) -> Configuration {
        let x0 = Variable::x_0();

        let sub = Vec::new();

        let t1_flat = t1.assoc_flatten();
        let t2_flat = t2.assoc_flatten();

        let init_active = Vec::from(&[AUT::new(x0.clone(),t1_flat.clone(), t2_flat.clone())]);

        Self::new(init_active,Vec::new(),sub,x0)
    }
    pub fn create_new_config(
        active: Vec<AUT>,
        store: Vec<AUT>,
        sub: Vec<Substitution>,
        var: Variable,
    ) -> Configuration {
        Configuration::new(
            active,
            store,
            sub,
            var
        )
    }

    pub fn to_generaliser(&self) -> Generalizer {
        let mut sub1 = Substitution::new();
        let mut sub2 = Substitution::new();

        //println!("{:?}",self.store);


        for aut in self.active.iter() {
            sub1.insert(&aut.x.clone(),&aut.t1.clone());
            sub2.insert(&aut.x.clone(),&aut.t2.clone());
        }

        for aut in self.store.iter() {
            sub1.insert(&aut.x.clone(),&aut.t1.clone());
            sub2.insert(&aut.x.clone(),&aut.t2.clone());
        }

        let mut  t = Term::Variable(Variable::x_0());

        for sub in self.sub.iter() {
            t = t.apply_substitution(sub);
        }

        Generalizer::new(&t, &sub1, &sub2)
    }
    
    

    

    // Helper methods for configuration creation

    pub fn with_fresh_aut_pair(
        &self,
        t1: Term,
        t1_prime: Term,
        t2: Term,
        t2_prime: Term,
    ) -> (Variable, Variable, AUT, AUT) {
        let x1 = Variable::fresh_variable();
        let x2 = Variable::fresh_variable();
        let aut1 = AUT::new(x1.clone(), t1, t1_prime);
        let aut2 = AUT::new(x2.clone(), t2, t2_prime);

        (x1, x2, aut1, aut2)
    }

    fn config_to_table(&self)->Vec<Vec<String>> {
        let max_len = self.active.len()
            .max(self.store.len())
            .max(self.sub.len());

        let mut rows = Vec::new();

        //Header
        rows.push(Vec::from(vec!["Active".to_string(),"Store".to_string(),"Sub".to_string()]));

        for i in 0..max_len {
            let active_aut = self.active.get(i)
                .map(|x| format!("{}", x))
                .unwrap_or_default();

            let store_aut = self.store.get(i)
                .map(|x| format!("{}", x))
                .unwrap_or_default();

            let sub = self.sub.get(i)
                .map(|x| format!("{}", x))
                .unwrap_or_default();

            rows.push(vec![active_aut, store_aut, sub]);
        }

        rows
    }


    pub fn draw(&self,name: &str,folder : &str) {
        let data = self.config_to_table();
        let rows = data.len();
        let cols = data[0].len();

        let max_width = self.active.iter().map(|x| x.to_string().len()).max().unwrap_or_default().max(self.store.iter().map(|x| x.to_string().len()).max().unwrap_or_default())
            .max(self.sub.iter().map(|x| x.to_string().len()).max().unwrap_or_default())
            .max(1);

        //let cell_w = 200+max_width*3;
        let cell_w = max_width*10+10;
        let cell_h  = 50;

        let width = (cols*cell_w) as u32;
        let height = (rows*cell_h) as u32;

        let mut img = RgbImage::new(width,height);

        let black = Rgb([0,0,0]);
        let white = Rgb([255,255,255]);

        //White background
        for p in img.pixels_mut() {
            *p = white;
        }

        // grid
        for c in 0..=cols{
            let x = (c*cell_w) as f32;
            draw_line_segment_mut(&mut img, (x,0.0), (x,height as f32), black);
        }

        // font
        let font = get_font();
        let scale = Scale::uniform(18.0);

        //text

        for (r,row) in data.iter().enumerate(){
            for (c,cell) in row.iter().enumerate(){
                let x = (c*cell_w+10) as i32;
                let y = (r*cell_h +10) as i32;
                draw_text_mut(&mut img,black,x,y,scale,&font,cell)
            }
        }

        img.save(format!("{}/{}.png",folder,name)).unwrap();

    }
    
}

impl Display for Configuration {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut active_string = String::new();
        let mut store_string = String::new();
        let mut sub_string = String::new();

        //Active set
        active_string.push('{');
        for aut in self.active.iter() {
            active_string.push_str(format!("{}, ",aut).as_str());
        }
        active_string.push('}');

        //Store set
        store_string.push('{');
        for aut in self.store.iter() {
            store_string.push_str(format!("{}, ",aut).as_str());
        }
        store_string.push('}');

        // Substitution

        if self.sub.is_empty(){
            sub_string.push_str("{}");
        }
        else{
            for sub in self.sub.iter() {
                sub_string.push_str(format!("{}",sub).as_str());
            }
        }




        //Total string

        let result = format!(
            "< {} |  {} | {} | {} >", active_string,store_string,sub_string, self.x0);

        write!(f, "{}", result)
    }
}