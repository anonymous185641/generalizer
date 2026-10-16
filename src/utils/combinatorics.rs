

pub struct Combinatorics;

impl Combinatorics {
    pub fn combinations<T: Clone>(arr: &[T], k: usize) -> Vec<Vec<T>> {
        let mut result = Vec::new();
        let mut combo = Vec::new();
        Self::generate_combinations(arr, k, 0, &mut combo, &mut result);
        result
    }

    fn generate_combinations<T: Clone>(
        arr: &[T],
        k: usize,
        start: usize,
        combo: &mut Vec<T>,
        result: &mut Vec<Vec<T>>,
    ) {
        if combo.len() == k {
            result.push(combo.clone());
            return;
        }

        for i in start..arr.len() {
            combo.push(arr[i].clone());
            Self::generate_combinations(arr, k, i + 1, combo, result);
            combo.pop();
        }
    }

    pub fn split_vector<T: Clone>(
        base_vector: &[T],
        index_vec: &[usize],
    ) -> (Vec<T>, Vec<T>) {
        let mut selected = Vec::new();
        let mut remaining = Vec::new();

        for (i, item) in base_vector.iter().enumerate() {
            if index_vec.contains(&i) {
                selected.push(item.clone());
            } else {
                remaining.push(item.clone());
            }
        }

        (selected, remaining)
    }
}