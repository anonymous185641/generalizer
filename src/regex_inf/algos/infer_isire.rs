use crate::regex_inf::regex::regex::Regex;
use crate::regex_inf::sample::sample::Sample;
use crate::regex_inf::soa::soa::SOA;

pub struct InferIsire {}


impl InferIsire {
    pub fn i_isire(sample: &Sample) -> Regex<String>{
        let mut soa = SOA::build_from_sample(sample);

        //println!("condended: {:?}",Dot::with_config(&soa.soa, &[Config::EdgeNoLabel]));

        Self::infer_isire(&mut soa, &sample)
    }

    pub fn infer_isire(soa: &mut SOA, sample: &Sample) -> Regex<String>{
        todo!()
    }
}