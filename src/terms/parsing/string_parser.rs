use pest::iterators::Pair;
use pest::Parser;
use crate::terms::function::{Axioms, FunctionSignature, Signature};
use crate::terms::parsing::parser::{GrammarParser, Rule};

pub fn parse_string(unparsed_string:&String) ->  Result<(Signature, String, String),String> {

    match GrammarParser::parse(Rule::PEST_FILE, unparsed_string) {
        Ok(ref mut got_pairs)=>{

            let mut sig = Vec::new();
            let mut t1 = String::new();
            let mut t2 = String::new();
            for pair in got_pairs{
                if pair.as_rule() == Rule::DCL_FUNCTION{
                    let f = parse_function(&pair)?;
                    sig.push(f);
                }
                if pair.as_rule() == Rule::PROBLEM{
                    (t1,t2) = parse_problem(&pair)?;
                }
            }
            Ok((sig,t1,t2))
        },
        Err(e)=>{

            return Err(e.to_string());
        }
    }
}



pub fn parse_function(unparsed_function: &Pair<Rule>) -> Result<FunctionSignature,String> {
    let pairs_inner = unparsed_function.clone().into_inner();
    let mut name = String::from("");
    let mut arity: usize = 0;
    let mut axioms = Vec::new();

    for pair in pairs_inner {
        if pair.as_rule() == Rule::DCL_NAME{
            name = pair.as_str().to_string();
        }
        if pair.as_rule() == Rule::DCL_ARITY{
            let arity_str = pair.as_str().trim();
            arity = arity_str.parse().unwrap();
        }
        //axioms later...

        if pair.as_rule() == Rule::DCL_AXIOMS{
            let axiom_str = pair.as_str().trim();

            if axiom_str == "S"{
                if arity!=0{
                    panic!("A special constant must be of arity 0");
                }
                axioms.push(Axioms::SpecialConst);
            }

            if axiom_str.contains("A") {
                if arity<2{
                    panic!("Associative functions must be of arity greater or equal than 2");
                }

                axioms.push(Axioms::A);
            }

            if axiom_str.contains("C"){
                if arity<2{
                    panic!("Commutative functions must be of arity greater or equal than 2");
                }

                axioms.push(Axioms::C);
            }

            if axiom_str.contains("U") && arity>0{
                if arity<2{
                    panic!("Functions having unit element must be of arity greater or equal than 2");
                }
                axioms.push(Axioms::U);
            }

        }


    }
    Ok(FunctionSignature::new(name, arity, axioms))
}

pub fn parse_problem(unparsed_problem: &Pair<Rule>) -> Result<(String,String),String> {
    let pair_problem = unparsed_problem.clone().into_inner();

    let mut t1 = String::new();
    let mut t2 = String::new();

    for pair in pair_problem {
        if pair.as_rule() == Rule::TERM1{
            t1 = pair.as_str().to_string();
        }

        if pair.as_rule() == Rule::TERM2{
            t2 = pair.as_str().to_string();
        }
    }

    Ok((t1,t2))
}

