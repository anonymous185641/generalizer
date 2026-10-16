
#[allow(dead_code)]
pub fn remove_first<T: PartialEq>(vec: Vec<T>, value: T) -> Vec<T> where T: Clone{
    let mut temp = vec.clone();
    if let Some(pos) = vec.iter().position(|x| *x == value) {
        temp.remove(pos);
    }
    temp
}

pub fn extract_inner(expr: &str, func: &str) -> String {
    if expr.starts_with(func) {
        let inner = expr[func.len()..].trim();
        if inner.starts_with('(') && inner.ends_with(')') {
            return inner[1..inner.len() - 1].trim().to_string();
        }
    }
    panic!("Malformed expression: {}", expr);
}


pub fn split_binary_op(expr: &str, op: &str) -> (String, String) {
    let inner = extract_inner(expr, op);
    let mut depth = 0;
    let mut split_index = None;

    for (i, c) in inner.chars().enumerate() {
        match c {
            '(' => depth += 1,
            ')' => depth -= 1,
            ',' if depth == 0 => {
                split_index = Some(i);
                break;
            }
            _ => {}
        }
    }

    if let Some(idx) = split_index {
        let left = inner[..idx].trim().to_string();
        let right = inner[idx + 1..].trim().to_string();
        (left, right)
    } else {
        panic!("Failed to split binary operation: {}", expr);
    }

}

pub fn have_common_element<T: PartialEq+Clone>(a: &Vec<T>, b: &Vec<T>) -> bool {
   for item in a {
       if b.contains(item) {
           return true;
       }
   }

    false
}

pub fn min_max_vec<T: PartialOrd+Clone>(vec: &Vec<T>) -> (T,T){
    let mut min = vec[0].clone();
    let mut max = vec[0].clone();
    for item in vec{
        if *item<min{
            min = item.clone();
        }
        if *item>max{
            max = item.clone();
        }
    }

    (min,max)
}


pub fn duration_secs_trunc(dur: std::time::Duration) -> f64 {
    (dur.as_secs_f64()*1000.0).trunc()/1000.0
}