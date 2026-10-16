use crate::interactions::syntax::interaction::Interaction;
use crate::regex_inf::regex::regex::Regex;

impl Interaction{
    /* ---------------------------------------------------------
    * Interaction Combinatorial Cardinality
    * ---------------------------------------------------------*/

    pub fn interaction_combinatorial_cardinality(&self) -> u128 {
        match self {
            Interaction::Empty | Interaction::Action(_) | Interaction::Vp(_,_) => 1,

             Interaction::Loop(_,box1) => {
                Self::interaction_combinatorial_cardinality(&*box1.clone())+1
            }

            Interaction::Alt(box1,box2) => {

                let v = vec![*box1.clone(),*box2.clone()];
                v.iter().map(Self::interaction_combinatorial_cardinality).sum()
            }

            Interaction::Seq(box1,box2) => {// To check again

                let v = vec![*box1.clone(),*box2.clone()];
                v.iter().map(Self::interaction_combinatorial_cardinality).product()
            }

            Interaction::Par(box1,box2)|Interaction::Tensor(box1,box2) => {
                let v = vec![*box1.clone(),*box2.clone()];
                let mut sizes = vec![1u128];
                let mut lens = vec![0usize];

                for e in v.iter() {
                    let cc = Self::interaction_combinatorial_cardinality(e);
                    let elen = Self::interaction_expr_len(e);

                    let mut next_sizes = Vec::new();
                    let mut next_lens = Vec::new();

                    for (&s, &l) in sizes.iter().zip(&lens) {
                        let ways = Regex::binom(l + elen, l);
                        next_sizes.push(s * cc * ways);
                        next_lens.push(l + elen);
                    }

                    sizes = next_sizes;
                    lens = next_lens;
                }
                sizes.into_iter().sum()
            }
            _=>{
                panic!("Unsupported interaction type");
            }
        }
    }

    fn interaction_expr_len(int: &Interaction) -> usize {
        match int {
            Interaction::Empty | Interaction::Action(_) | Interaction::Vp(_,_) => 1,
            Interaction::Seq(box1,box2)
            | Interaction::Par(box1,box2)
            |Interaction::Tensor(box1,box2)=>{
                let v = vec![*box1.clone(),*box2.clone()];
                v.iter().map(Self::interaction_expr_len).sum()
            },
            Interaction::Alt(box1,box2) =>{
                let v = vec![*box1.clone(),*box2.clone()];
                v.iter().map(Self::interaction_expr_len).max().unwrap_or(0)
            },
            Interaction::Loop(_,box1) => Self::interaction_expr_len(&*box1.clone()),
            _ => 0,
        }
    }
}