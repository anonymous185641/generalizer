





/*
Copyright 2020 Erwan Mahe (github.com/erwanM974)

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/





use crate::interactions::syntax::interaction::Interaction;

pub fn special_get_recursive_alt_frags(interaction : &Interaction) -> Vec<Interaction> {
    let mut frags : Vec<Interaction> = Vec::new();
    match interaction.clone() {
        Interaction::Alt(box1, box2) => {
            let i1 = *box1;
            let i2 = *box2;
            frags.extend( special_get_recursive_alt_frags(&i1));
            frags.extend( special_get_recursive_alt_frags(&i2));
        },
        _ => {
            frags.push(interaction.clone());
        }
    }
    return frags;
}

pub fn special_get_recursive_par_frags(interaction : &Interaction) -> Vec<Interaction> {
    let mut frags : Vec<Interaction> = Vec::new();
    match interaction.clone() {
        Interaction::Par(box1, box2) => {
            let i1 = *box1;
            let i2 = *box2;
            frags.extend( special_get_recursive_par_frags(&i1));
            frags.extend( special_get_recursive_par_frags(&i2));
        },
        _ => {
            frags.push(interaction.clone());
        }
    }
    return frags;
}

#[allow(dead_code)]
pub fn special_get_recursive_seq_frags(interaction : &Interaction) -> Vec<Interaction> {
    let mut frags : Vec<Interaction> = Vec::new();
    match interaction.clone() {
        Interaction::Seq(box1, box2) => {
            let i1 = *box1;
            let i2 = *box2;
            frags.extend( special_get_recursive_seq_frags(&i1));
            frags.extend( special_get_recursive_seq_frags(&i2));
        },
        _ => {
            frags.push(interaction.clone());
        }
    }
    return frags;
}

pub fn special_get_recursive_tensor_frags(interaction : &Interaction) -> Vec<Interaction> {
    let mut frags : Vec<Interaction> = Vec::new();
    match interaction.clone() {
        Interaction::Tensor(box1, box2) => {
            let i1 = *box1;
            let i2 = *box2;
            frags.extend( special_get_recursive_tensor_frags(&i1));
            frags.extend( special_get_recursive_tensor_frags(&i2));
        },
        _ => {
            frags.push(interaction.clone());
        }
    }
    return frags;
}






