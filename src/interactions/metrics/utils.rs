use crate::interactions::syntax::interaction::Interaction;

impl Interaction{
    pub fn compositions(total: usize, parts: usize) -> Vec<Vec<usize>> {
        fn rec(
            remaining: usize,
            parts: usize,
            current: &mut Vec<usize>,
            out: &mut Vec<Vec<usize>>,
        ) {
            if parts == 1 {
                current.push(remaining);
                out.push(current.clone());
                current.pop();
                return;
            }

            for v in 1..=(remaining - parts + 1) {
                current.push(v);
                rec(remaining - v, parts - 1, current, out);
                current.pop();
            }
        }

        let mut out = Vec::new();
        rec(total, parts, &mut Vec::new(), &mut out);
        out
    }
    
    pub fn binom(n: usize, k: usize) -> usize {
        if k > n { return 0; }
        if k == 0 || k == n { return 1; }

        let k = k.min(n - k);
        let mut res = 1usize;
        for i in 0..k {
            res = res * (n - i) / (i + 1);
        }
        res
    }
}