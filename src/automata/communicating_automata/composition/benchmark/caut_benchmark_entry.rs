use std::time::Duration;
use crate::automata::communicating_automata::composition::communicating_automata_to_regex::automata_to_regex_algos::AutomataToRegexAlgo;
use crate::utils::misc::duration_secs_trunc;

#[derive(Debug, Clone)]
pub struct BenchmarkCompositionEntry{
    pub use_case_name: String,
    //pub with_interleaving: bool,
    pub number_automata: usize,
    pub number_messages: usize,
    pub range_automata_size: (usize,usize),
    pub brzozowski_entry: BrzozowskiCompositionEntry,
    pub pn2regex_entry: Pn2regexCompositionEntry,
}


impl BenchmarkCompositionEntry {


    pub fn new(use_case_name: String,
               // with_interleaving: bool,
               number_automata: usize,
               number_messages: usize,
               range_automata_size: (usize,usize),
               brzozowski_entry: BrzozowskiCompositionEntry,
               pn2regex_entry: Pn2regexCompositionEntry) ->Self{
        BenchmarkCompositionEntry{use_case_name,
            //with_interleaving,
            number_automata,
            number_messages,
            range_automata_size,
            brzozowski_entry,
            pn2regex_entry}
    }

    pub fn create_csv_record(&self)->[String;11]{
        let brzozowski_record = self.brzozowski_entry.create_csv_record();
        let pn2regex_record = self.pn2regex_entry.create_csv_record();

       // let with_interleaving = if self.with_interleaving{ "Y" }else{ "N" };

        let mut rec = [
            self.use_case_name.clone(),
            self.number_automata.to_string(),
            self.number_messages.to_string(),
        format!("[{},{}]",self.range_automata_size.0,self.range_automata_size.1)];

        let rec_vec: Vec<String> = rec
            .iter()
            .chain(brzozowski_record.iter())
            .chain(pn2regex_record.iter())
            .cloned()
            .collect();

        let arr: [String;11] = rec_vec.try_into().expect("Incorrect vector length");

        arr
    }

}

#[derive(Debug, Clone)]
pub struct BrzozowskiCompositionEntry{
    pub range_size_local_interactions:(usize,usize),
    pub total_time: Option<Duration>,
    pub size_global_interaction: Option<usize>,
}
impl BrzozowskiCompositionEntry{
    pub fn new(range_size_local_interactions:(usize,usize), total_time: Option<Duration>, size_global_interactions: Option<usize>) ->Self{
        BrzozowskiCompositionEntry{range_size_local_interactions, total_time, size_global_interaction: size_global_interactions }
    }

    pub fn create_csv_record(&self)->[String;3]{
        if self.total_time.is_none()||self.size_global_interaction.is_none(){
            return [format!("[{},{}]",self.range_size_local_interactions.0,self.range_size_local_interactions.1),//range of the size of the interactions
            "--".to_string(),"--".to_string()];
        }

        [format!("[{},{}]",self.range_size_local_interactions.0,self.range_size_local_interactions.1),//range of the size of the interactions
        self.total_time.map(|t| duration_secs_trunc(t).to_string()).unwrap_or_default(),//time of the interaction composition
        self.size_global_interaction.map(|t| t.to_string()).unwrap_or_default()] //size of the global interaction
    }
}

#[derive(Debug, Clone)]
pub struct PetrifyData{
    pub range_petrify_time: (Duration, Duration),
    pub range_petri_net_size: (usize,usize),
}
#[derive(Debug, Clone)]
pub struct Pn2regexCompositionEntry{
    pub petrify_data: Option<PetrifyData>,
    pub range_size_local_interactions:Option<(usize,usize)>,
    pub total_time: Option<Duration>,
    pub size_global_interactions: Option<usize>,
}

impl Pn2regexCompositionEntry{
    pub fn new(petrify_data: Option<PetrifyData>, range_size_local_interactions:Option<(usize,usize)>, total_time: Option<Duration>, size_global_interactions: Option<usize>) ->Self{
        Pn2regexCompositionEntry{petrify_data,range_size_local_interactions, total_time,size_global_interactions}
    }

    pub fn create_csv_record(&self)->[String;4]{
        if self.petrify_data.is_none()||self.range_size_local_interactions.is_none()||self.total_time.is_none()||self.size_global_interactions.is_none(){
            ["--".to_string(),"--".to_string(),"--".to_string(),"--\\\\".to_string()]
        }else{
            [//format!("[{},{}]",self.petrify_data.clone().unwrap().range_petrify_time.0.as_secs_f64().to_string(),self.petrify_data.clone().unwrap().range_petrify_time.1.as_secs_f64().to_string()),
                format!("[{},{}]",self.petrify_data.clone().unwrap().range_petri_net_size.0,self.petrify_data.clone().unwrap().range_petri_net_size.1),
                format!("[{},{}]",self.range_size_local_interactions.unwrap().0,self.range_size_local_interactions.unwrap().1),
                self.total_time.map(|t| duration_secs_trunc(t).to_string()).unwrap_or_default(),
                format!("{}\\\\",self.size_global_interactions.map(|t| t.to_string()).unwrap_or_default()),
            ]
        }
        
    }
}