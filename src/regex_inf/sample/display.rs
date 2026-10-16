use std::fmt;
use crate::regex_inf::sample::sample::Sample;

impl fmt::Display for Sample {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let mut buff = format!("Alphabet: {:?}\n",self.alphabet);
        buff.push_str(&format!("Words:\n"));
        for w in &self.sample{
            if w.is_empty(){
                buff.push_str(&format!("{}\n","\"\""));
                continue;
            }
            else{
                buff.push_str(&format!("{}\n",w));
            }
        }
        write!(f,"{}",buff )

    }
}