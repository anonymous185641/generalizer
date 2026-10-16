use std::fs::File;


#[derive(Clone,Debug)]
pub struct Exp2Line {
    pub global_interaction_name: String,
    pub global_interaction_size: usize,
    pub global_interaction_nb_lfs: usize,
    pub tot_nb_multitraces: usize,
    pub nb_selected_multitraces: usize,
    pub tot_nb_results: usize,
    pub nb_results_after_normalization: usize,
    pub duration: f64
}

impl Exp2Line {
    pub fn new(
        global_interaction_name: &String,
        global_interaction_size: usize,
        global_interaction_nb_lfs: usize,
        tot_nb_multitraces: usize,
        nb_selected_multitraces: usize,
        tot_nb_results: usize,
        nb_results_after_normalization: usize,
        duration: f64
    ) -> Self {
        Self {
            global_interaction_name: global_interaction_name.clone(),
            global_interaction_size,
            global_interaction_nb_lfs,
            tot_nb_multitraces,
            nb_selected_multitraces,
            tot_nb_results,
            nb_results_after_normalization,
            duration
        }
    }
}


#[derive(Clone,Debug)]
pub struct Exp2Output {
    pub results_benchmark: Vec<Exp2Line>,
}

impl Exp2Output {
    pub fn new() -> Self {
        Self{results_benchmark: Vec::new()}
    }

    pub fn add_line(&mut self, line: &Exp2Line){
        self.results_benchmark.push(line.clone());
    }

    pub fn sort(&mut self) {
        self.results_benchmark
            .sort_by(|a, b| a.global_interaction_name.cmp(&b.global_interaction_name));
    }
    pub fn to_csv(&self,parent_dir: &str,millis:bool,timout: f64){
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
            "Total Number of multitraces",
            "Number of selected multitraces",
            "Total Number of results",
            "Number of results after normalization",
            "Duration"]);

        for line in &self.results_benchmark{

            let _ = wtr.write_record(&[line.global_interaction_name.clone(),
                line.global_interaction_size.to_string(),
                line.global_interaction_nb_lfs.to_string(),
                line.tot_nb_multitraces.to_string(),
                line.nb_selected_multitraces.to_string(),
                line.tot_nb_results.to_string(),
                line.nb_results_after_normalization.to_string(),
                format!("{}\\\\",Self::duration_print(line.duration,timout))]);
        }

        let _ = wtr.flush();
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
