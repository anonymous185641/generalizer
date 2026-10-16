

use crate::interactions::syntax::interaction::Interaction;

pub fn special_fold_recursive_alt_frags(frags : &mut Vec<Interaction>) -> Interaction {
    let frag_num = frags.len();
    if frag_num == 2 {
        let i2 = frags.pop().unwrap();
        let i1 = frags.pop().unwrap();
        return Interaction::Alt( Box::new(i1.clone()), Box::new(i2.clone()) );
    } else if frag_num == 1 {
        return frags.pop().unwrap().clone();
    } else if frag_num == 0 {
        return Interaction::Empty
    } else {
        let i1 = frags.remove(0);
        return Interaction::Alt( Box::new(i1.clone()), Box::new( special_fold_recursive_alt_frags(frags) ) );
    }
}

pub fn special_fold_recursive_tensor_frags(frags : &mut Vec<Interaction>) -> Interaction {
    let frag_num = frags.len();
    if frag_num == 2 {
        let i2 = frags.pop().unwrap();
        let i1 = frags.pop().unwrap();
        return Interaction::Tensor( Box::new(i1.clone()), Box::new(i2.clone()) );
    } else if frag_num == 1 {
        return frags.pop().unwrap().clone();
    } else if frag_num == 0 {
        return Interaction::Empty
    } else {
        let i1 = frags.remove(0);
        return Interaction::Tensor( Box::new(i1.clone()), Box::new( special_fold_recursive_tensor_frags(frags) ) );
    }
}

pub fn special_fold_recursive_seq_frags(frags : &mut Vec<Interaction>) -> Interaction {
    let frag_num = frags.len();
    if frag_num == 2 {
        let i2 = frags.pop().unwrap();
        let i1 = frags.pop().unwrap();
        return Interaction::Seq( Box::new(i1.clone()), Box::new(i2.clone()) );
    } else if frag_num == 1 {
        return frags.pop().unwrap().clone();
    } else if frag_num == 0 {
        return Interaction::Empty
    } else {
        let i1 = frags.remove(0);
        return Interaction::Seq( Box::new(i1.clone()), Box::new( special_fold_recursive_seq_frags(frags) ) );
    }
}

pub fn special_fold_recursive_par_frags(frags : &mut Vec<Interaction>) -> Interaction {
    let frag_num = frags.len();
    if frag_num == 2 {
        let i2 = frags.pop().unwrap();
        let i1 = frags.pop().unwrap();
        return Interaction::Par( Box::new(i1.clone()), Box::new(i2.clone()) );
    } else if frag_num == 1 {
        return frags.pop().unwrap().clone();
    } else if frag_num == 0 {
        return Interaction::Empty
    } else {
        let i1 = frags.remove(0);
        return Interaction::Par( Box::new(i1.clone()), Box::new( special_fold_recursive_par_frags(frags) ) );
    }
}

