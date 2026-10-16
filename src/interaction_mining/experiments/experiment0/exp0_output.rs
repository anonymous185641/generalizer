use std::fs::File;

#[derive(Clone,Debug)]
pub enum Exp0CompVerdict{
    Success(bool,f64),
    Timout,
    Failure,
}

#[derive(Clone,Debug)]
pub struct Exp0Line{
    pub global_interaction_name: String,
    pub global_interaction_size: usize,
    pub nb_local_traces: usize,
    pub regex_inf_time: Vec<f64>,
    pub comp_time: Vec<Exp0CompVerdict>,
}

impl Exp0Line{
    pub fn new(global_interaction_name: &String,
                global_interaction_size: usize,
                nb_local_traces: usize,
               regex_inf_time: &Vec<f64>,
               comp_time: &Vec<Exp0CompVerdict>,
    )-> Self{
        Self{
        global_interaction_name: global_interaction_name.clone(),
        global_interaction_size,
        nb_local_traces,
        regex_inf_time: regex_inf_time.clone(),
        comp_time: comp_time.clone(),
        }
    }


    pub fn normalize_durations(&self,millis:bool)->Exp0Line{
        let mut new_regex_inf_time = Vec::new();
        let mut new_comp_time = Vec::new();

        if millis{
            for t in self.regex_inf_time.iter(){
                new_regex_inf_time.push((t.clone()*1000000.0).trunc()/1000.0);
            }

            for verd in self.comp_time.iter(){
                if let Exp0CompVerdict::Success(b,t) = &verd{
                    new_comp_time.push(Exp0CompVerdict::Success(*b,(t.clone()*1000000.0).trunc()/1000.0));
                }
                else{
                    new_comp_time.push(verd.clone())
                }
            }

        }
        else{
            for t in self.regex_inf_time.iter(){
            new_regex_inf_time.push((t.clone()*1000.0).trunc()/1000.0);
            }

            for verd in self.comp_time.iter(){
                if let Exp0CompVerdict::Success(b,t) = &verd{
                    new_comp_time.push(Exp0CompVerdict::Success(*b,(t.clone()*1000.0).trunc()/1000.0));
                }
                else{
                    new_comp_time.push(verd.clone())
                }
            }
        }

        Self::new(&self.global_interaction_name,
        self.global_interaction_size,
        self.nb_local_traces,
        &new_regex_inf_time,
        &new_comp_time)
    }
}


#[derive(Clone,Debug)]
pub struct Exp0Output{
    pub results_benchmark: Vec<Exp0Line>,
}

impl Exp0Output{
    pub fn new() -> Self {
        Self{results_benchmark: Vec::new()}
    }

    pub fn add_line(&mut self, line: &Exp0Line){
        self.results_benchmark.push(line.clone());
    }

    pub fn sort(&mut self) {
        self.results_benchmark
            .sort_by(|a, b| a.global_interaction_name.cmp(&b.global_interaction_name));
    }
    pub fn to_csv(&self,parent_dir: &str,millis:bool){
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
        let mut duration1_str =  String::from("Soa2Chare inference time");
        let mut duration2_str =  String::from("Soa2Sore inference time");
        let mut duration3_str =  String::from("CRX inference time");
        let mut duration4_str =  String::from("GenICHARE inference time");
        let mut duration5_str =  String::from("ConMiner inference time");
        let mut duration6_str =  String::from("Soa2Soire inference time");

        if millis {
            duration1_str.push_str("(ms)");
            duration2_str.push_str("(ms)");
            duration3_str.push_str("(ms)");
            duration4_str.push_str("(ms)");
            duration5_str.push_str("(ms)");
            duration6_str.push_str("(ms)");
        }
        else{
            duration1_str.push_str("(s)");
            duration2_str.push_str("(s)");
            duration3_str.push_str("(s)");
            duration4_str.push_str("(s)");
            duration5_str.push_str("(s)");
            duration6_str.push_str("(s)");
        }
        let _ = wtr.write_record(&["Global interaction",
            "Size of the global interaction",
            "Number of local traces",
            &duration1_str,
            &duration2_str,
            &duration3_str,
            &duration4_str,
            &duration5_str,
            &duration6_str]);

        for line in &self.results_benchmark{

            let _ = wtr.write_record(&[line.global_interaction_name.clone(),
                line.global_interaction_size.to_string(),
                line.nb_local_traces.to_string(),
                Self::custom_expect(&line.comp_time[0]),
                Self::custom_expect(&line.comp_time[1]),
                Self::custom_expect(&line.comp_time[2]),
                Self::custom_expect(&line.comp_time[3]),
                Self::custom_expect(&line.comp_time[4]),
                format!("{}\\\\",Self::custom_expect(&line.comp_time[5]))]);
        }

        let _ = wtr.flush();
    }


    pub fn to_csv_interleaving_only(&self,parent_dir: &str,millis:bool){
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
        let mut duration1_str =  String::from("GenICHARE inference time");
        let mut duration2_str =  String::from("ConMiner inference time");
        let mut duration3_str =  String::from("Soa2Soire composition time");

        if millis {
            duration1_str.push_str("(ms)");
            duration2_str.push_str("(ms)");
            duration3_str.push_str("(ms)");
        }
        else{
            duration1_str.push_str("(s)");
            duration2_str.push_str("(s)");
            duration3_str.push_str("(s)");
        }
        let _ = wtr.write_record(&["Global interaction",
            "Size of the global interaction",
            "Number of local traces",
            &duration1_str,
            &duration2_str,
            &duration3_str,]);

        for line in &self.results_benchmark{

            let _ = wtr.write_record(&[line.global_interaction_name.clone(),
                line.global_interaction_size.to_string(),
                line.nb_local_traces.to_string(),
                Self::custom_expect(&line.comp_time[3]),
                Self::custom_expect(&line.comp_time[4]),//????
                format!("{}\\\\",Self::custom_expect(&line.comp_time[5]))]);
        }

        let _ = wtr.flush();
    }
    pub fn custom_expect(op: &Exp0CompVerdict)->String{
        match op{
            Exp0CompVerdict::Success(b,x) => {
                if *b==true{
                    format!("{}(\\greencheck)",x)
                }
                else{
                    format!("{}(\\redcross)",x)
                }
            },
            Exp0CompVerdict::Timout => "timout(\\redcross)".to_string(),
            Exp0CompVerdict::Failure => "failure(\\redcross)".to_string(),
        }
    }
}
