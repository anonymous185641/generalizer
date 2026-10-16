use std::collections::HashMap;
use crate::regex_inf::regex::regex::Regex;
use crate::regex_inf::sample::sample::Sample;

impl Regex<String>{

    /* ---------------------------------------------------------
     * Length counting: |L(E)_i|
     * ---------------------------------------------------------*/

    fn length_counts(
        re: &Regex<String>,
        max_len: usize,
        memo: &mut HashMap<usize, Vec<u128>>,
    ) -> Vec<u128> {
        let key = re as *const _ as usize;
        if let Some(v) = memo.get(&key) {
            return v.clone();
        }

        let mut res = vec![0u128; max_len + 1];

        match re {
            Regex::Empty => {}
            Regex::Epsilon => res[0] = 1,
            Regex::Literal(_) => {
                if max_len >= 1 { res[1] = 1; }
            }

            Regex::Or(v) => {
                for e in v {
                    let c = Self::length_counts(e, max_len, memo);
                    for i in 0..=max_len {
                        res[i] += c[i];
                    }
                }
            }

            Regex::Concat(v) => {
                res[0] = 1;
                for e in v {
                    let c = Self::length_counts(e, max_len, memo);
                    let mut next = vec![0u128; max_len + 1];
                    for i in 0..=max_len {
                        for j in 0..=max_len - i {
                            next[i + j] += res[i] * c[j];
                        }
                    }
                    res = next;
                }
            }

            Regex::Int(v) => {
                // Interleaving = shuffle
                let mut cur = vec![0u128; max_len + 1];
                cur[0] = 1;

                for e in v {
                    let c = Self::length_counts(e, max_len, memo);
                    let mut next = vec![0u128; max_len + 1];
                    for i in 0..=max_len {
                        for j in 0..=max_len - i {
                            let ways = Self::binom(i + j, i);
                            next[i + j] += cur[i] * c[j] * ways;
                        }
                    }
                    cur = next;
                }
                res = cur;
            }

            Regex::Qm(e) => {
                let c = Self::length_counts(e, max_len, memo);
                res[0] = 1;
                for i in 0..=max_len {
                    res[i] += c[i];
                }
            }
            /*
            Regex::Plus(e) => {
                let c = Self::length_counts(e, max_len, memo);
                for i in 1..=max_len {
                    for k in 1..=i {
                        res[i] += c[k] * res[i - k];
                    }
                }
            }

             */
            Regex::Plus(e) => {
                // e+ = e · e*
                let c = Self::length_counts(e, max_len, memo);

                // Build counts for e* (Kleene star) up to max_len
                let mut star = vec![0u128; max_len + 1];
                star[0] = 1;
                for i in 1..=max_len {
                    for k in 1..=i {
                        star[i] += c[k] * star[i - k];
                    }
                }

                // Convolution: plus[i] = Σ_{k=1..i} c[k] * star[i-k]
                for i in 1..=max_len {
                    for k in 1..=i {
                        res[i] += c[k] * star[i - k];
                    }
                }
            }
            Regex::Kleene(e) => {
                res[0] = 1;
                let c = Self::length_counts(e, max_len, memo);
                for i in 1..=max_len {
                    for k in 1..=i {
                        res[i] += c[k] * res[i - k];
                    }
                }
            }

            Regex::SRC | Regex::SNK => {}
        }

        memo.insert(key, res.clone());
        res
    }

    /* ---------------------------------------------------------
     * Language Size
     * ---------------------------------------------------------*/

    pub fn language_size(&self, alphabet_len: usize) -> usize {
        let max_len = 2 * alphabet_len + 1;
        let mut memo = HashMap::new();
        let counts = Self::length_counts(self, max_len, &mut memo);
        counts.iter().map(|v| *v as usize).sum()
    }

    /* ---------------------------------------------------------
     * Data Cost (MDL)
     * ---------------------------------------------------------*/

    pub fn data_cost(&self, sample: &Sample) -> f64 {
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
        let counts = Self::length_counts(self, max_len, &mut memo);

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

    /* ---------------------------------------------------------
     * Combinatorial Cardinality (Definition 4)
     * ---------------------------------------------------------*/

    pub fn combinatorial_cardinality(&self) -> u128 {
        match self {
            Regex::Empty | Regex::Epsilon | Regex::Literal(_) => 1,

            Regex::Qm(e) | Regex::Kleene(e) => {
                Self::combinatorial_cardinality(e)+1
            }

             Regex::Plus(e)  => {
                Self::combinatorial_cardinality(e)
            }

            Regex::Or(v) => {
                v.iter().map(Self::combinatorial_cardinality).sum()
            }

            Regex::Concat(v) => {
                v.iter().map(Self::combinatorial_cardinality).product()
            }

            Regex::Int(v) => {
                let mut sizes = vec![1u128];
                let mut lens = vec![0usize];

                for e in v {
                    let cc = Self::combinatorial_cardinality(e);
                    let elen = Self::expr_len(e);

                    let mut next_sizes = Vec::new();
                    let mut next_lens = Vec::new();

                    for (&s, &l) in sizes.iter().zip(&lens) {
                        let ways = Self::binom(l + elen, l);
                        next_sizes.push(s * cc * ways);
                        next_lens.push(l + elen);
                    }

                    sizes = next_sizes;
                    lens = next_lens;
                }
                sizes.into_iter().sum()
            }

            Regex::SRC | Regex::SNK => 1,
        }
    }

    fn expr_len(re: &Regex<String>) -> usize {
        match re {
            Regex::Literal(_) => 1,
            Regex::Concat(v) | Regex::Int(v) => v.iter().map(Self::expr_len).sum(),
            Regex::Or(v) => v.iter().map(Self::expr_len).max().unwrap_or(0),
            Regex::Qm(e) | Regex::Plus(e) | Regex::Kleene(e) => Self::expr_len(e),
            _ => 0,
        }
    }
}