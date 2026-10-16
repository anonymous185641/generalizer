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

pub fn transfo_flush_right(interaction : &Interaction) -> Vec<Interaction> {
    match interaction {
        &Interaction::Alt(ref i1, ref i2) => {
            match **i1 {
                Interaction::Alt(ref i11,ref i12) => {
                    return vec![Interaction::Alt( i11.clone(), Box::new(Interaction::Alt(i12.clone(), i2.clone())) )];
                },
                _ => {}
            }
        },
        &Interaction::Tensor(ref i1, ref i2) => {
            match **i1 {
                Interaction::Tensor(ref i11,ref i12) => {
                    return vec![Interaction::Tensor( i11.clone(), Box::new(Interaction::Tensor(i12.clone(), i2.clone())) )];
                },
                _ => {}
            }
        },
        &Interaction::Seq(ref i1, ref i2) => {
            match **i1 {
                Interaction::Seq(ref i11,ref i12) => {
                    return vec![Interaction::Seq( i11.clone(), Box::new(Interaction::Seq(i12.clone(), i2.clone())) )];
                },
                _ => {}
            }
        },
        &Interaction::Par(ref i1, ref i2) => {
            match **i1 {
                Interaction::Par(ref i11,ref i12) => {
                    return vec![Interaction::Par( i11.clone(), Box::new(Interaction::Par(i12.clone(), i2.clone())) )];
                },
                _ => {}
            }
        },
        _ => {}
    }
    return vec![]
}

/*
pub fn flush_left(interaction : &Interaction) -> Vec<Interaction> {
    match interaction {
        &Interaction::Alt(ref i1, ref i2) => {
            match **i2 {
                Interaction::Alt(ref i21,ref i22) => {
                    return vec![ Interaction::Alt( Box::new(Interaction::Alt(i1.clone(), i21.clone())), i22.clone() ) ];
                },
                _ => {}
            }
        },
        &Interaction::Tensor(ref i1, ref i2) => {
            match **i2 {
                Interaction::Tensor(ref i21,ref i22) => {
                    return vec![ Interaction::Tensor( Box::new(Interaction::Tensor(i1.clone(), i21.clone())), i22.clone() ) ];
                },
                _ => {}
            }
        },
        &Interaction::Seq(ref i1, ref i2) => {
            match **i2 {
                Interaction::Seq(ref i21,ref i22) => {
                    return vec![ Interaction::Seq( Box::new(Interaction::Seq(i1.clone(), i21.clone())), i22.clone() ) ];
                },
                _ => {}
            }
        },
        &Interaction::Par(ref i1, ref i2) => {
            match **i2 {
                Interaction::Par(ref i21,ref i22) => {
                    return vec![ Interaction::Par( Box::new(Interaction::Par(i1.clone(), i21.clone())), i22.clone() ) ];
                },
                _ => {}
            }
        },
        &Interaction::CoReg(ref cr1, ref i1, ref i2) => {
            match **i2 {
                Interaction::CoReg(ref cr2, ref i21,ref i22) => {
                    if cr1 == cr2 {
                        return vec![ Interaction::CoReg( cr1.clone(), Box::new(Interaction::CoReg(cr1.clone(), i1.clone(), i21.clone())), i22.clone() ) ];
                    }
                },
                _ => {}
            }
        },
        _ => {}
    }
    return vec![];
}*/