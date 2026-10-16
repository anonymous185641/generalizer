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





use crate::interactions::syntax::iat_canonize::transformation::transfokind::InteractionTransformationKind;
use crate::interactions::syntax::iat_canonize::transformation::transfores::InteractionTransformationResult;
use crate::interactions::syntax::interaction::Interaction;
use crate::interactions::syntax::position::Position;


pub fn get_all_transformations_rec(transfos : &Vec<InteractionTransformationKind>,
                                   interaction : &Interaction) -> Vec<InteractionTransformationResult> {
    let mut results = get_all_transformations_inner(transfos,interaction);
    match interaction {
        &Interaction::Empty => {
            // ***
        }, &Interaction::Action(_) => {
            // ***
        }, &Interaction::Vp(_,_) => {
            // ***
        }, &Interaction::Tensor(ref i1, ref i2) => {
            for left_transfo in get_all_transformations_rec(transfos,i1) {
                results.push( InteractionTransformationResult::new(left_transfo.kind,
                                                                   Position::Left(Box::new(left_transfo.position)),
                                                                   Interaction::Tensor(Box::new(left_transfo.result),i2.clone())
                ) );
            }
            for right_transfo in get_all_transformations_rec(transfos,i2) {
                results.push( InteractionTransformationResult::new(right_transfo.kind,
                                                                   Position::Right(Box::new(right_transfo.position)),
                                                                   Interaction::Tensor(i1.clone(), Box::new(right_transfo.result))
                ) );
            }
        }, &Interaction::Seq(ref i1, ref i2) => {
            for left_transfo in get_all_transformations_rec(transfos,i1) {
                results.push( InteractionTransformationResult::new(left_transfo.kind,
                                                                   Position::Left(Box::new(left_transfo.position)),
                                                                   Interaction::Seq(Box::new(left_transfo.result),i2.clone())
                ) );
            }
            for right_transfo in get_all_transformations_rec(transfos,i2) {
                results.push( InteractionTransformationResult::new(right_transfo.kind,
                                                                   Position::Right(Box::new(right_transfo.position)),
                                                                   Interaction::Seq(i1.clone(), Box::new(right_transfo.result))
                ) );
            }
        },
        &Interaction::Par(ref i1, ref i2) => {
            for left_transfo in get_all_transformations_rec(transfos,i1) {
                results.push( InteractionTransformationResult::new(left_transfo.kind,
                                                                   Position::Left(Box::new(left_transfo.position)),
                                                                   Interaction::Par(Box::new(left_transfo.result),i2.clone())
                ) );
            }
            for right_transfo in get_all_transformations_rec(transfos,i2) {
                results.push( InteractionTransformationResult::new(right_transfo.kind,
                                                                   Position::Right(Box::new(right_transfo.position)),
                                                                   Interaction::Par(i1.clone(), Box::new(right_transfo.result))
                ) );
            }
        }, &Interaction::Alt(ref i1, ref i2) => {
            for left_transfo in get_all_transformations_rec(transfos,i1) {
                results.push( InteractionTransformationResult::new(left_transfo.kind,
                                                                   Position::Left(Box::new(left_transfo.position)),
                                                                   Interaction::Alt(Box::new(left_transfo.result),i2.clone())
                ) );
            }
            for right_transfo in get_all_transformations_rec(transfos,i2) {
                results.push( InteractionTransformationResult::new(right_transfo.kind,
                                                                   Position::Right(Box::new(right_transfo.position)),
                                                                   Interaction::Alt(i1.clone(), Box::new(right_transfo.result))
                ) );
            }
        },&Interaction::CoReg(ref cr, ref i1, ref i2) => {
            for left_transfo in get_all_transformations_rec(transfos,i1) {
                results.push( InteractionTransformationResult::new(left_transfo.kind,
                                                                   Position::Left(Box::new(left_transfo.position)),
                                                                   Interaction::CoReg(cr.clone(), Box::new(left_transfo.result),i2.clone())
                ) );
            }
            for right_transfo in get_all_transformations_rec(transfos,i2) {
                results.push( InteractionTransformationResult::new(right_transfo.kind,
                                                                   Position::Right(Box::new(right_transfo.position)),
                                                                   Interaction::CoReg(cr.clone(), i1.clone(), Box::new(right_transfo.result))
                ) );
            }
        }, &Interaction::Loop(ref kind, ref i1) => {
            for left_transfo in get_all_transformations_rec(transfos,i1) {
                results.push( InteractionTransformationResult::new(left_transfo.kind,
                                                                   Position::Left(Box::new(left_transfo.position)),
                                                                   Interaction::Loop(kind.clone(), Box::new(left_transfo.result))
                ) );
            }
        }
    }
    return results;
}

fn get_all_transformations_inner(transfos : &Vec<InteractionTransformationKind>,
                                 interaction : &Interaction) -> Vec<InteractionTransformationResult> {

    let mut results : Vec<InteractionTransformationResult> = Vec::new();
    for transfo_kind in transfos {
        let new_transfos : Vec<InteractionTransformationResult> = transfo_kind.get_transformation()(interaction)
            .into_iter().map(|x| InteractionTransformationResult::new((*transfo_kind).clone(), Position::Epsilon(None), x)).collect();
        results.extend(new_transfos);
    }
    return results;
}

