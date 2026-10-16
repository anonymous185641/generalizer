use std::collections::HashMap;
use crate::interactions::syntax::interaction::Interaction;
use crate::regex_inf::sample::sample::Sample;

impl Interaction{
    pub fn interaction_data_cost(&self, sample: &Sample) -> f64 {
        let mut len_hist = HashMap::<usize, usize>::new();
        for s in &sample.sample {
            let len = match sample.separator {
                Some(c) => s.split(c).count(),
                None => s.len(),
            };
            *len_hist.entry(len).or_default() += 1;
        }

        let max_len = 2 * sample.alphabet.len() + 1;
        let mut memo = HashMap::new();
        let counts = Self::interaction_length_counts(self, max_len, &mut memo);

        let mut cost = 0.0;
        for i in 0..=max_len {
            if let Some(si) = len_hist.get(&i) {
                if counts[i] > 0 {
                    cost += 2.0 * (i as f64).log2()
                        + ((counts[i] as f64) / (*si as f64)).log2();
                }
            }
        }
        cost
    }
}