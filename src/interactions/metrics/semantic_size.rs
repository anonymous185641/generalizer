use std::collections::HashMap;
use crate::interactions::syntax::interaction::Interaction;

impl Interaction {

    pub fn traces_of_len(
        i: usize,
        it: &Interaction,
        memo: &mut HashMap<(usize, Interaction), usize>,
    ) -> usize {
        if let Some(v) = memo.get(&(i, it.clone())) {
            return *v;
        }

        let result = match it {
            // ||∅||^i = 0
            Interaction::Empty => {
                if i == 0 { 1 } else { 0 }
            },

            // ||a||^i = 1 if i=1 else 0
            Interaction::Action(_) => {
                if i == 1 { 1 } else { 0 }
            }

            // ||vp(a,b)||^i = 1 if i=2 else 0
            Interaction::Vp(_, _) => {
                if i == 2 { 1 } else { 0 }
            }

            // ||alt(l1,l2)||^i = ||l1||^i + ||l2||^i
            Interaction::Alt(l1, l2) => {
                Self::traces_of_len(i, l1, memo) + Self::traces_of_len(i, l2, memo)
            }

            // Par and Tensor are the same
            Interaction::Par(l1, l2) | Interaction::Tensor(l1, l2) => {
                let mut sum = 0;
                for j1 in 0..=i {
                    let j2 = i - j1;
                    sum += Self::binom(i, j1)
                        * Self::traces_of_len(j1, l1, memo)
                        * Self::traces_of_len(j2, l2, memo);
                }
                sum
            }

            // Seq
            Interaction::Seq(l1, l2) => {
                let i1 = *l1.clone();
                let i2 = *l2.clone();

                let t1 = i1.get_lifelines();
                let t2 = i2.get_lifelines();

                if !t1.is_disjoint(&t2) {
                    // shuffle like Par
                    let mut sum = 0;
                    for j1 in 0..=i {
                        let j2 = i - j1;
                        sum += Self::binom(i, j1)
                            * Self::traces_of_len(j1, l1, memo)
                            * Self::traces_of_len(j2, l2, memo);
                    }
                    sum
                } else {
                    // independent → same as Par in your lemma
                    Self::traces_of_len(i, &Interaction::Par(l1.clone(), l2.clone()), memo)
                }
            }

            // LoopS(l)
            Interaction::Loop(_,body) => {
                let mut total = 0;

                if i == 0{
                    return 1;
                }
                // N = number of repetitions (must be ≥ 1)
                for n in 1..=i {
                    // distribute i across n segments: j1 + ... + jn = i, ji > 0
                    total += Self::compositions(i, n)
                        .into_iter()
                        .map(|parts| {
                            parts.into_iter()
                                .map(|len| Self::traces_of_len(len, body, memo))
                                .product::<usize>()
                        })
                        .sum::<usize>();
                }

                total
            },
            _=>{
                panic!("Unsupported interaction type");
            }
        };

        memo.insert((i, it.clone()), result);
        result
    }

    pub fn count_traces(i: usize, it: &Interaction) -> usize {
        let mut memo = HashMap::new();
        Self::traces_of_len(i, it, &mut memo)
    }

    pub fn semantic_size(&self) -> usize {
        let acts = self.get_all_actions();
        let n = 2 * acts.len() + 1;

        let mut memo = HashMap::new();
        let mut sum = 0;

        for i in 0..=n {
            sum += Self::traces_of_len(i, self, &mut memo);
        }

        sum
    }
}