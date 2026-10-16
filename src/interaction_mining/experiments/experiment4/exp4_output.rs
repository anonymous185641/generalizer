use std::fs::File;


#[derive(Clone,Debug)]
pub struct Exp4Line {
    pub global_interaction_name: String,
    pub global_interaction_size: usize,
    pub global_interaction_nb_lfs: usize,
    pub global_interaction_language_size: usize,
    pub tot_nb_multitraces: usize,
    pub tot_duration: f64,
    pub results_sizes : Vec<usize>,
    pub results_lang_sizes : Vec<usize>,
}

impl Exp4Line {
    pub fn new(
        global_interaction_name: &String,
        global_interaction_size: usize,
        global_interaction_nb_lfs: usize,
        global_interaction_language_size: usize,
        tot_nb_multitraces: usize,
        tot_duration: f64,
        results_sizes : Vec<usize>,
        results_lang_sizes : Vec<usize>
    ) -> Self {
        Self {
            global_interaction_name: global_interaction_name.clone(),
            global_interaction_size,
            global_interaction_nb_lfs,
            global_interaction_language_size,
            tot_nb_multitraces,
            tot_duration,
            results_sizes,
            results_lang_sizes
        }
    }
}


#[derive(Clone,Debug)]
pub struct Exp4Output {
    pub results_benchmark: Vec<Exp4Line>,
}

impl Exp4Output {
    pub fn new() -> Self {
        Self{results_benchmark: Vec::new()}
    }

    pub fn add_line(&mut self, line: &Exp4Line){
        self.results_benchmark.push(line.clone());
    }

    pub fn sort(&mut self) {
        self.results_benchmark
            .sort_by(|a, b| a.global_interaction_name.cmp(&b.global_interaction_name));
    }
    pub fn to_csv(&self,parent_dir: &str){
        let file_name = format!("{}/results.csv",parent_dir);
        let file = if let Some(f) = File::create(&file_name).ok(){
            f
        }
        else{
            panic!("Error creating CSV file");
        };

        //let mut wtr = Writer::from_writer(file);
        let mut wtr = csv::WriterBuilder::new()
            .delimiter(b'&')
            //.quote_style(csv::QuoteStyle::NonNumeric)
            .from_writer(file);

        //Writing

        let _ = wtr.write_record(&["Global interaction",
            "Size of the global interaction",
            "Number of lifelines",
            "Size of the semantics",
            "Total Number of multitraces",
            "Number of selected multitraces",
            "Mining Duration",
            "Results size",
            "results language size"]);

        for line in &self.results_benchmark{
            let sizes_str = Self::size_print(line.results_sizes.clone());
            let lang_sizes_str = Self::lang_size_print(line.results_lang_sizes.clone());
            let _ = wtr.write_record(&[line.global_interaction_name.clone(),
                line.global_interaction_size.to_string(),
                line.global_interaction_nb_lfs.to_string(),
                line.global_interaction_language_size.to_string(),
                line.tot_nb_multitraces.to_string(),
                line.tot_duration.to_string(),
                sizes_str[0].clone(),
                sizes_str[1].clone(),
                sizes_str[2].clone(),
                sizes_str[3].clone(),
                sizes_str[4].clone(),
                sizes_str[5].clone(),
                sizes_str[6].clone(),
                sizes_str[7].clone(),
                sizes_str[8].clone(),
                lang_sizes_str[0].clone(),
                lang_sizes_str[1].clone(),
                lang_sizes_str[2].clone(),
                lang_sizes_str[3].clone(),
                lang_sizes_str[4].clone(),
                lang_sizes_str[5].clone(),
                lang_sizes_str[6].clone(),
                lang_sizes_str[7].clone(),
                format!("{}\\\\",lang_sizes_str[8])]);

        }

        let _ = wtr.flush();
    }


    fn size_print(size: Vec<usize>)->Vec<String>{
        let mut size_str = Vec::new();
        for s in size{
            size_str.push(s.to_string());
        }
        size_str
    }

    fn lang_size_print(size: Vec<usize>)->Vec<String>{
        let mut size_str = Vec::new();
        for s in size{
            size_str.push(s.to_string());
        }
        size_str
    }

    fn duration_print(duration: f64,timout: f64)->String{
        if duration >= timout{
            "timeout".to_string()
        }
        else{
            format!("{}",duration)
        }
    }
}
