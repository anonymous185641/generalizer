use std::cmp::min;
use std::collections::HashMap;
use crate::interactions::syntax::action::Action;
use crate::interactions::syntax::interaction::Interaction;

impl Interaction{
    /// Substitution cost for leaf payloads
    fn action_cost(a: &Action, b: &Action) -> usize {
        if a == b { 0 } else { 1 }
    }

    fn edit_distance_rec(
        a: &Interaction,
        b: &Interaction,
        memo: &mut HashMap<(Interaction, Interaction), usize>,
    ) -> usize {
        if let Some(v) = memo.get(&(a.clone(), b.clone())) {
            return *v;
        }

        let result = match (a, b) {
            // Identical nodes
            (x, y) if x == y => 0,

            // Empty cases
            (Interaction::Empty, other) => other.size(),
            (other, Interaction::Empty) => other.size(),

            // Action ↔ Action
            (Interaction::Action(a1), Interaction::Action(a2)) => Self::action_cost(a1, a2),

            // Vp ↔ Vp (ordered pair!)
            (Interaction::Vp(a1, b1), Interaction::Vp(a2, b2)) => {
                Self::action_cost(a1, a2) + Self::action_cost(b1, b2)
            }

            // LoopS ↔ LoopS
            (Interaction::Loop(_,x), Interaction::Loop(_,y)) => 1 + Self::edit_distance_rec(x, y, memo),

            // Binary structured nodes of the SAME kind
            (Interaction::Seq(a1, b1), Interaction::Seq(a2, b2))
            | (Interaction::Alt(a1, b1), Interaction::Alt(a2, b2))
            | (Interaction::Par(a1, b1), Interaction::Par(a2, b2))
            | (Interaction::Tensor(a1, b1), Interaction::Tensor(a2, b2)) => {
                1 + Self::edit_distance_rec(a1, a2, memo) + Self::edit_distance_rec(b1, b2, memo)
            }

            // Different variants → substitute root + best effort on children
            (x, y) => {
                1 + min(
                    x.size() + y.size(), // delete x, insert y
                    match (x, y) {
                        // Try aligning children if arities match
                        (Interaction::Loop(_,xc), Interaction::Loop(_,yc)) => Self::edit_distance_rec(xc, yc, memo),
                        (
                            Interaction::Seq(x1, x2)
                            | Interaction::Alt(x1, x2)
                            | Interaction::Par(x1, x2)
                            | Interaction::Tensor(x1, x2),
                            Interaction::Seq(y1, y2)
                            | Interaction::Alt(y1, y2)
                            | Interaction::Par(y1, y2)
                            | Interaction::Tensor(y1, y2),
                        ) => Self::edit_distance_rec(x1, y1, memo) + Self::edit_distance_rec(x2, y2, memo),
                        _ => x.size() + y.size(),
                    },
                )
            }
        };

        memo.insert((a.clone(), b.clone()), result);
        result
    }

    /// Memoized tree edit distance
    pub fn edit_distance(a: &Interaction, b: &Interaction) -> usize {
        let mut memo = HashMap::new();
        Self::edit_distance_rec(a, b, &mut memo)
    }
    
    
}