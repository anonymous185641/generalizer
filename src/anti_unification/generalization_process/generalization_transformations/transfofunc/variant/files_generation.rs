use crate::anti_unification::generalization_process::generalization_transformations::transfofunc::variant::variant::Variant;
use crate::terms::substitution::variable::Variable;

impl Variant {
        pub fn generate_maude_file_interaction(additional_variables: &Vec<Variable>)->String{
            let vars_string = additional_variables.iter().map(|v| v.name()).collect::<Vec<String>>().join(" ");
            format!("
                fmod INTERACTION is
                including NAT .

                sort Interaction .

                op Empty : -> Interaction [ctor] .
                op Action : Nat Nat Nat Nat -> Interaction [ctor] .
                op Vp : Interaction Interaction -> Interaction [ctor] .

                op seq : Interaction Interaction -> Interaction [assoc].
                op tensor : Interaction Interaction -> Interaction .
                op par : Interaction Interaction -> Interaction .
                op alt : Interaction Interaction -> Interaction .
                op loopS : Interaction -> Interaction .

                sort Lifeline .
                sort LifelineList .

                subsort Lifeline < Nat .
                subsort Lifeline < LifelineList .

                op _,_ : Lifeline Lifeline -> LifelineList [assoc comm] .

                op coreg[_](_,_) : LifelineList Interaction Interaction -> Interaction .
                *** Equations ***
                ***eq loopS(Empty) = Empty [].***
                vars x1 x2 x3 : [Interaction] .
                var L : [LifelineList] .
                vars {} : [Interaction] .

                eq seq(x1,Empty) = x1 [variant] .
                eq seq(Empty,x1) = x1 [variant] .
                eq par(x1,Empty) = x1 [variant] .
                eq par(Empty,x1) = x1 [variant] .
                eq coreg[L](x1,Empty) = x1 [variant] .
	            eq coreg[L](Empty,x1) = x1 [variant] .
                eq alt(x1,x1) = x1 [variant] .
                eq seq(loopS(x1),loopS(x1)) = loopS(x1) [variant] .
                eq alt(seq(x1,x2),seq(x1,x3)) = seq(x1,alt(x2,x3)) [variant] .
            endfm
            ",vars_string)
        }

    pub fn generate_maude_file_interaction_loop_alt(additional_variables: &Vec<Variable>)->String{
        let vars_string = additional_variables.iter().map(|v| v.name()).collect::<Vec<String>>().join(" ");
        format!("
                fmod INTERACTION is
                including NAT .

                sort Interaction .

                op Empty : -> Interaction [ctor] .
                op Action : Nat Nat Nat Nat -> Interaction [ctor] .
                op Vp : Interaction Interaction -> Interaction [ctor] .

                op seq : Interaction Interaction -> Interaction [assoc].
                op tensor : Interaction Interaction -> Interaction .
                op par : Interaction Interaction -> Interaction .
                op alt : Interaction Interaction -> Interaction [assoc].
                op loopS : Interaction -> Interaction .

                sort Lifeline .
                sort LifelineList .

                subsort Lifeline < Nat .
                subsort Lifeline < LifelineList .

                op _,_ : Lifeline Lifeline -> LifelineList [assoc comm] .

                op coreg[_](_,_) : LifelineList Interaction Interaction -> Interaction .
                *** Equations ***
                ***eq loopS(Empty) = Empty [].***
                vars x1 x2 x3 : [Interaction] .
                var L : [LifelineList] .
                vars {} : [Interaction] .


                eq seq(x1,Empty) = x1 [variant] .
                eq seq(Empty,x1) = x1 [variant] .
                eq par(x1,Empty) = x1 [variant] .
                eq par(Empty,x1) = x1 [variant] .
                eq coreg[L](x1,Empty) = x1 [variant] .
	            eq coreg[L](Empty,x1) = x1 [variant] .
                eq alt(x1,x1) = x1 [variant] .
                eq seq(loopS(x1),loopS(x1)) = loopS(x1) [variant] .
                eq alt(seq(x1,x2),seq(x1,x3)) = seq(x1,alt(x2,x3)) [variant] .
                eq loopS(alt(x1,Empty)) = loopS(x1) [variant] .
            endfm
            ",vars_string)
    }


    pub fn generate_python_script_interaction(maude_file: &String)->String{
        format!(r#"
import maude
import sys
import random

maude.init()
maude.input({:?})
m = maude.getCurrentModule()

t = m.parseTerm(sys.argv[1])

variants = t.get_variants(True)

for (t,subs) in variants:
    print(t,subs,sep='+++')
        "#,maude_file)
    }
}




















