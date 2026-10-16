use crate::regex_inf::regex::regex::Regex;
use crate::regex_inf::sample::sample::Sample;
use crate::regex_inf::soa::soa::SOA;


pub struct I_KOIRE{}

impl I_KOIRE{
    /*
    pub fn i_koire(sample: &Sample, N: usize)-> Regex<String>{
        let mut candidates_regex:HashSet<Regex<String>>  = HashSet::new();
        let k_max = sample.max_number_of_occurences();
        for k in 1..=k_max{
            for n in 1..=N {
                let  koa = KOA::gen_koa(sample,k);
                let  (soa,position_map )= KOA::mark_koa(&koa);
                let  new_sample = sample.mark_sample(&position_map);
                let regex = Self::gen_koire(&soa,&new_sample);
                candidates_regex.insert(regex);
            }
        }

        Regex::best_regex(&candidates_regex)
    }

     */


    pub fn gen_koire(soa: &SOA, sample: &Sample)-> Regex<String>{
        todo!();
    }

}


