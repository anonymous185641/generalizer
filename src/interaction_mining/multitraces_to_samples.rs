use std::collections::{HashMap, HashSet};
use image::ImageEncoder;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::{thread_rng, SeedableRng};
use crate::interactions::syntax::action::Action;
use crate::interactions::syntax::colocalizations::CoLocalizations;
use crate::interactions::syntax::general_context::GeneralContext;
use crate::interactions::syntax::traces::multitrace::MultiTrace;
use crate::regex_inf::sample::sample::Sample;

impl Sample{
    pub fn multitraces_to_samples_with_ratio(mult: &Vec<(CoLocalizations,MultiTrace)>,trace_ratio:f64)->Vec<(usize,Sample)>{
        if mult.is_empty() {
            return Vec::new();
        }

        // We interpret "total number of traces" as the number of entries in `mult`,
        // because `multitraces_to_samples` iterates `for (_, b) in mult { ... }`.
        let total = mult.len();

        // Helper: compute samples for the first `k` traces (a prefix of `mult`),
        // using the same logic as `multitraces_to_samples`.
        let samples_for_prefix = |k: usize| -> Vec<(usize, Sample)> {
            let k = k.clamp(1, total);

            // Pick k distinct random elements (without replacement)
            let seed: u64 = 12345;
            let mut rng = StdRng::seed_from_u64(seed);

            let mut idxs: Vec<usize> = (0..total).collect();
            idxs.shuffle(&mut rng);
            let chosen: Vec<&(CoLocalizations, MultiTrace)> =
                idxs.into_iter().take(k).map(|i| &mult[i]).collect();


            let mut ret = vec![];

            let coloc = {
                let mut temp = Vec::new();
                let mult0 = chosen[0].0.clone();

                for id in mult0.locs_lf_ids {
                    temp.push(id.first().unwrap().clone());
                }
                temp
            };

            for lf_id in &coloc {
                let mut sample = Vec::new();
                let mut alphabet = Vec::new();

                for (_, b) in chosen.iter().copied() {
                    let (seq, act) = Self::join_and_collect((*b[*lf_id]).to_vec());
                    sample.push(seq.clone());
                    alphabet.extend(act.clone());
                }

                alphabet.sort();
                alphabet.dedup();

                sample.sort();
                sample.dedup();

                let samp_coll = Sample { alphabet, sample, separator: Some('.') };
                ret.push((*lf_id, samp_coll));
            }

            ret
        };

        let k = ((trace_ratio * total as f64).ceil() as usize).clamp(1, total);
        samples_for_prefix(k)
    }
    pub fn multitraces_to_samples_with_proportions(mult: &Vec<(CoLocalizations,MultiTrace)>)->Vec<Vec<(usize,Sample)>>{
        if mult.is_empty() {
            return Vec::new();
        }

        // We interpret "total number of traces" as the number of entries in `mult`,
        // because `multitraces_to_samples` iterates `for (_, b) in mult { ... }`.
        let total = mult.len();

        // Requested buckets: 2%, 5%, 10%, 50%, 75%, 100%
        let proportions: [f64; 6] = [0.02, 0.05, 0.10, 0.50, 0.75, 1.00];

        // Helper: compute samples for the first `k` traces (a prefix of `mult`),
        // using the same logic as `multitraces_to_samples`.
        let samples_for_prefix = |k: usize| -> Vec<(usize, Sample)> {
            let k = k.clamp(1, total);

            // Pick k distinct random elements (without replacement)
            let mut rng = thread_rng();
            let mut idxs: Vec<usize> = (0..total).collect();
            idxs.shuffle(&mut rng);
            let chosen: Vec<&(CoLocalizations, MultiTrace)> =
                idxs.into_iter().take(k).map(|i| &mult[i]).collect();


            let mut ret = vec![];

            let coloc = {
                let mut temp = Vec::new();
                let mult0 = chosen[0].0.clone();

                for id in mult0.locs_lf_ids {
                    temp.push(id.first().unwrap().clone());
                }
                temp
            };

            for lf_id in &coloc {
                let mut sample = Vec::new();
                let mut alphabet = Vec::new();

                for (_, b) in chosen.iter().copied() {
                    let (seq, act) = Self::join_and_collect((*b[*lf_id]).to_vec());
                    sample.push(seq.clone());
                    alphabet.extend(act.clone());
                }

                alphabet.sort();
                alphabet.dedup();

                sample.sort();
                sample.dedup();

                let samp_coll = Sample { alphabet, sample, separator: Some('.') };
                ret.push((*lf_id, samp_coll));
            }

            ret
        };

        let mut buckets: Vec<Vec<(usize, Sample)>> = Vec::with_capacity(proportions.len());

        for p in proportions {
            // "only X% of total traces": take a prefix of size ceil(p * total).
            // (ceil keeps small-but-nonzero percentages from turning into 0 too often)
            let k = ((p * total as f64).ceil() as usize).clamp(1, total);
            buckets.push(samples_for_prefix(k));
        }

        buckets
    }

    pub fn multitraces_to_samples(mult: &Vec<(CoLocalizations,MultiTrace)>)->Vec<(usize,Sample)>{

        let mut ret = vec![];

        let mut coloc = {
            let mut temp = Vec::new();
            let mult0 = mult[0].0.clone();

            for id in mult0.locs_lf_ids{
                temp.push(id.first().unwrap().clone());
            }
            temp
        };


        for lf_id in &coloc{
            let mut sample = Vec::new();
            let mut alphabet = Vec::new();

            for (_,b) in mult{
                let (seq,act) = Self::join_and_collect((*b[*lf_id]).to_vec());

                sample.push(seq.clone());
                alphabet.extend(act.clone());
            }

            alphabet.sort();
            alphabet.dedup();

            sample.sort();
            sample.dedup();

            let samp_coll = Sample{alphabet,sample,separator:Some('.')};

            ret.push((*lf_id,samp_coll));
        }

        ret
    }

    fn join_and_collect<T: ToString>(items: Vec<T>) -> (String, HashSet<String>) {
        // Convert all items to strings
        let strings: Vec<String> = items.into_iter().map(|x| x.to_string()).collect();

        // Build the dot-separated string
        let joined = strings.join(".");

        // Build the HashSet
        let set: HashSet<String> = strings.into_iter().collect();

        (joined, set)
    }
}