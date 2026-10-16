

/* ---------------------------------------------------------
    * Length counting: |L(E)_i|
    * ---------------------------------------------------------*/
use std::collections::HashMap;
use crate::interactions::syntax::interaction::Interaction;
use crate::regex_inf::regex::regex::Regex;


impl Interaction{
    /* ---------------------------------------------------------
    * Length counting: |sigma(int)_i|
    * ---------------------------------------------------------*/

    pub fn interaction_length_counts(
        int: &Interaction,
        max_len: usize,
        memo: &mut HashMap<usize, Vec<u128>>,
    ) -> Vec<u128> {
        let key = int as *const _ as usize;
        if let Some(v) = memo.get(&key) {
            return v.clone();
        }

        let mut res = vec![0u128; max_len + 1];

        match int {
            Interaction::Empty => {}
            Interaction::Action(_) => {
                if max_len >= 1 { res[1] = 1; }
            }
            Interaction::Vp(_,_)=>{
                if max_len >= 2 { res[2] = 1; }
            }
            Interaction::Alt(box1,box2) => {
                let v = vec![*box1.clone(),*box2.clone()];
                for e in v.iter() {
                    let c = Self::interaction_length_counts(e, max_len, memo);
                    for i in 0..=max_len {
                        res[i] += c[i];
                    }
                }
            }

            Interaction::Seq(box1,box2) => {

                let v = vec![*box1.clone(),*box2.clone()];
                res[0] = 1;
                for e in v.iter() {
                    let c = Self::interaction_length_counts(e, max_len, memo);
                    let mut next = vec![0u128; max_len + 1];
                    for i in 0..=max_len {
                        for j in 0..=max_len - i {
                            next[i + j] += res[i] * c[j];
                        }
                    }
                    res = next;
                }
            }

            Interaction::Par(box1,box2)|Interaction::Tensor(box1,box2) => {

                // Interleaving = shuffle

                let v = vec![*box1.clone(),*box2.clone()];
                let mut cur = vec![0u128; max_len + 1];
                cur[0] = 1;

                for e in v.iter() {
                    let c = Self::interaction_length_counts(e, max_len, memo);
                    let mut next = vec![0u128; max_len + 1];
                    for i in 0..=max_len {
                        for j in 0..=max_len - i {
                            let ways = Regex::binom(i + j, i);
                            next[i + j] += cur[i] * c[j] * ways;
                        }
                    }
                    cur = next;
                }
                res = cur;
            }

            Interaction::Loop(_,box1) => {

                let i1 = *box1.clone();

                res[0] = 1;
                let c = Self::interaction_length_counts(&i1, max_len, memo);
                for i in 1..=max_len {
                    for k in 1..=i {
                        res[i] += c[k] * res[i - k];
                    }
                }
            },
            _ =>{
                panic!("Unsupported interaction type");
            }

        }

        memo.insert(key, res.clone());
        res
    }

    /* ---------------------------------------------------------
     * Language Size| Semantics Size
     * ---------------------------------------------------------*/

    pub fn den_semantic_size_old(&self) -> usize {
        let max_len = 2 * self.get_all_actions().len() + 1;
        let mut memo = HashMap::new();
        let counts = Self::interaction_length_counts(self, max_len, &mut memo);
        counts.iter().map(|v| *v as usize).sum()
    }
}
