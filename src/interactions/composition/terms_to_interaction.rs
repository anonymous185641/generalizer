use crate::interactions::syntax::action::{Action, ActionType};
use crate::interactions::syntax::interaction::{Interaction, LoopKind};
use crate::interactions::syntax::util::special_fold_int::{special_fold_recursive_alt_frags, special_fold_recursive_par_frags, special_fold_recursive_seq_frags, special_fold_recursive_tensor_frags};
use crate::terms::term::Term;
use crate::utils::misc::{extract_inner, split_binary_op};

impl Term{
    pub fn to_interaction(&self)->Interaction{
        match &self{
            &Term::Variable(_)=> panic!("Cannot convert variable to interaction"),
            &Term::Function(f)=>{

                match (f.signature.arity){
                    0 => {
                        Interaction::parse_interaction(&f.signature.name)
                    },
                    1 => {
                        match f.signature.name.as_str(){
                            "loopS"=>{
                                Interaction::wrap_loop(&LoopKind::SStrictSeq,&f.args[0].to_interaction())
                            },
                            "loopP"=>{
                                Interaction::wrap_loop(&LoopKind::PInterleaving,&f.args[0].to_interaction())
                            },
                            "loopW"=>{
                                Interaction::wrap_loop(&LoopKind::WWeakSeq,&f.args[0].to_interaction())
                            },
                            "loopH"=>{
                                Interaction::wrap_loop(&LoopKind::HHeadFirstWS,&f.args[0].to_interaction())
                            }
                            _ => {
                                panic!("Invalid function name during the conversion of a term into an interaction")
                            }
                        }
                    },
                    2 => {
                        match f.signature.name.as_str(){
                            "seq"=>{
                                let mut temp: Vec<Interaction> = f.args.iter().map(|arg| arg.to_interaction()).collect();
                                special_fold_recursive_seq_frags(&mut temp)
                            },
                            "par"=>{
                                let mut temp: Vec<Interaction> = f.args.iter().map(|arg| arg.to_interaction()).collect();
                                special_fold_recursive_par_frags(&mut temp)                            },
                            "alt"=>{
                                let mut temp: Vec<Interaction> = f.args.iter().map(|arg| arg.to_interaction()).collect();
                                special_fold_recursive_alt_frags(&mut temp)                            },
                            "tensor"=>{
                                let mut temp: Vec<Interaction> = f.args.iter().map(|arg| arg.to_interaction()).collect();
                                special_fold_recursive_tensor_frags(&mut temp)                            },
                            _ => {
                                panic!("Invalid function name during the conversion of a term into an interaction")
                            }
                        }
                    },
                    _ =>{
                        println!("Arity {:?}",f.signature.arity);
                        println!("Args {:?}",f.args.len());
                        panic!("Invalid function arity during the conversion of a term into an interaction");
                    }
                }

            }
        }
    }
    /*
    pub fn to_interaction(&self)->Interaction{
        match &self{
            &Term::Variable(_)=> panic!("Cannot convert variable to interaction"),
            &Term::Function(f)=>{

                match (f.signature.arity,f.args.len()){
                    (0,0) => {
                        Interaction::parse_interaction(&f.signature.name)
                    },
                    (1,1) => {
                        match f.signature.name.as_str(){
                            "loopS"=>{
                                Interaction::wrap_loop(&f.args[0].to_interaction())
                            },
                            _ => {
                                panic!("Invalid function name during the conversion of a term into an interaction")
                            }
                        }
                    },
                    (2,2) => {
                        match f.signature.name.as_str(){
                            "seq"=>{
                                Interaction::wrap_binary(&f.args[0].to_interaction(),&f.args[1].to_interaction(),&Operator::Seq)
                            },
                            "par"=>{
                                Interaction::wrap_binary(&f.args[0].to_interaction(),&f.args[1].to_interaction(),&Operator::Par)
                            },
                            "alt"=>{
                                Interaction::wrap_binary(&f.args[0].to_interaction(),&f.args[1].to_interaction(),&Operator::Alt)
                            },
                            "tensor"=>{
                                Interaction::wrap_binary(&f.args[0].to_interaction(),&f.args[1].to_interaction(),&Operator::Tensor)
                            },
                            _ => {
                                panic!("Invalid function name during the conversion of a term into an interaction")
                            }
                        }
                    },
                    _ =>{
                        println!("Arity {:?}",f.signature.arity);
                        println!("Args {:?}",f.args.len());
                        panic!("Invalid function arity during the conversion of a term into an interaction");
                    }
                }

            }
        }
    }

     */



}



impl Interaction{
    pub fn parse_interaction(expr: &str) -> Interaction{
        let expr = expr.trim();

        if expr == "Empty" {
            Interaction::Empty
        }
        else if expr.starts_with("Action(") {
            /*
            let parts: Vec<&str> = extract_inner(expr, "Action")
                .split(',')
                .map(|s| s.trim())
                .collect();
    
             */
            let p0= extract_inner(expr, "Action");
            let p1 = p0.split(',');
            let p2 = p1.map(|s| s.trim()).filter(|s| !s.is_empty());
            let parts: Vec<&str> = p2.collect();
            // println!("problematic parts: {:?}", parts);
            if parts.len() != 4 {
                panic!("Invalid Action format: {}", expr);
            }

            let ty = if parts[2].to_string().eq(&"0".to_string()) {
                ActionType::Emission
            }
            else{
                ActionType::Reception
            };
            let gate_id = if parts[3].to_string().eq(&"0".to_string()){
                None
            }
            else{
                Some(parts[3].parse::<usize>().ok().unwrap())
            };

            let act = Action::new_with_id_raw(parts[0].parse().unwrap(),parts[1].parse().unwrap(),ty,gate_id);
            Interaction::Action(act)

        } else if expr.starts_with("Vp(") {
            let (left, right) = split_binary_op(expr, "Vp");

            if let (Interaction::Action(a1),Interaction::Action(a2)) = (Self::parse_interaction(&left),Self::parse_interaction(&right)){
                Interaction::Vp(a1.clone(),a2.clone())
            }
            else{
                panic!("Invalid Vp format: {}", expr);
            }

        } 
        else {
            panic!("Unsupported Interaction expression: {}", expr);
        }
    }
}