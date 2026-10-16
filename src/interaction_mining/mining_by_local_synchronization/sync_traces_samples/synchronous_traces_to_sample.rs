use crate::interactions::syntax::interaction::Interaction;
use crate::interactions::syntax::traces::sync_trace::SynchronousTrace;
use crate::regex_inf::sample::sample::Sample;

impl Interaction{
    pub fn sync_traces_to_sample(sync_traces: &Vec<SynchronousTrace>)-> Sample {

        let mut sample: Vec<String> = Vec::new();

        for trace in sync_traces{
            let mut word = String::from("");

            for vp in trace.iter(){
                let vp_string = format!("{}",vp);
                if word.len()>0{
                    word.push('.');
                }
                word+= vp_string.as_str();
            }

            sample.push(word);
        }


        Sample::new_from_set(&sample,Some('.'))
    }
}