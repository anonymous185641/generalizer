use crate::interactions::composition::guideline::Guideline;
use crate::interactions::syntax::interaction::Interaction;
use crate::interactions::syntax::operators::Operator;
use crate::terms::function::{Axioms, Function, FunctionSignature};
use crate::terms::term::Term;


impl Interaction{



    pub fn to_term(&self,guideline: &Guideline,axioms: &Vec<Axioms>)->Term{
        match &self.clone(){
            Interaction::Empty =>{
                let sig = FunctionSignature::new(self.to_string(),0,vec![]);
                let f = Function::new(&sig,&vec![]);
                Term::Function(f)
            },
            Interaction::Vp(a1,a2) =>{

                let label = format!("Vp({},{})",a1.action_label(),a2.action_label());

                let sig = FunctionSignature::new_with_lfs(label,0,vec![],vec![a1.lf_id,a2.lf_id]);
                let f = Function::new(&sig,&vec![]);
                Term::Function(f)
            },
            Interaction::Action(a)=>{

                let sig = if let Some(g) = a.gate_id{

                    if guideline.map.contains_key(&g){
                        FunctionSignature::new_with_lfs(g.to_string(),0,vec![Axioms::SpecialConst],vec![a.lf_id])
                    }
                    else{
                        FunctionSignature::new_with_lfs(a.action_label(),0,vec![],vec![a.lf_id])
                    }

                }
                else{
                     FunctionSignature::new_with_lfs(a.action_label(),0,vec![],vec![a.lf_id])
                };


                let f = Function::new(&sig,&vec![]);
                Term::Function(f)
            },
            Interaction::Loop(kind,box1)=>{
               let i1 = *box1.clone();
                let op = Operator::Loop(kind.clone());
                
                let loop_sig: FunctionSignature = FunctionSignature::new(op.to_string(), 1, vec![]);


                let f = Function::new(&loop_sig, &vec![i1.to_term(&guideline,axioms)]);

                Term::Function(f)
            }
            Interaction::Seq(box1,box2)=>{
                let i1 = *box1.clone();
                let i2 = *box2.clone();

                let seq_sig: FunctionSignature = if axioms.contains(&Axioms::A) && axioms.contains(&Axioms::U){
                     FunctionSignature::new("seq".to_string(), 2, vec![Axioms::A, Axioms::U])
                }
                else if axioms.contains(&Axioms::A) && !axioms.contains(&Axioms::U){
                     FunctionSignature::new("seq".to_string(), 2, vec![Axioms::A])
                }
                else{
                     FunctionSignature::new("seq".to_string(), 2, vec![])
                };



                let f  = Function::new(&seq_sig, &vec![i1.to_term(guideline,axioms), i2.to_term(guideline,axioms)]);

                Term::Function(f)
            },
            Interaction::Par(box1,box2)=>{
                let i1 = *box1.clone();
                let i2 = *box2.clone();


                //let par_sig: FunctionSignature = FunctionSignature::new("par".to_string(), 2, vec![Axioms::A, Axioms::U]);
                let par_sig: FunctionSignature = FunctionSignature::new("par".to_string(), 2, axioms.clone());

                let f  = Function::new(&par_sig, &vec![i1.to_term(guideline,axioms), i2.to_term(guideline,axioms)]);

                Term::Function(f)
            },
            Interaction::CoReg(cr,box1,box2)=>{
                let i1 = *box1.clone();
                let i2 = *box2.clone();

                let lfs_as_string = cr.iter().map(|lf| format!("{}",lf)).collect::<Vec<String>>().join(",");
                let coreg_label = format!("coreg[{}]",lfs_as_string);
                
                let co_reg_sig: FunctionSignature = FunctionSignature::new(coreg_label, 2, vec![Axioms::A, Axioms::U]);
                let f  = Function::new(&co_reg_sig, &vec![i1.to_term(guideline,axioms), i2.to_term(guideline,axioms)]);
                Term::Function(f)
            }
            Interaction::Alt(box1,box2)=>{
                let i1 = *box1.clone();
                let i2 = *box2.clone();

                let alt_sig: FunctionSignature = if axioms.contains(&Axioms::A) && axioms.contains(&Axioms::C){
                    FunctionSignature::new("alt".to_string(), 2, vec![Axioms::A,Axioms::C])
                }
                else if axioms.contains(&Axioms::A) && !axioms.contains(&Axioms::C){
                    FunctionSignature::new("alt".to_string(), 2, vec![Axioms::A])
                }
                else if !axioms.contains(&Axioms::A) && axioms.contains(&Axioms::C){
                    FunctionSignature::new("alt".to_string(), 2, vec![Axioms::C])
                }
                else{
                    FunctionSignature::new("alt".to_string(), 2, vec![])
                };

                //let alt_sig: FunctionSignature = FunctionSignature::new("alt".to_string(), 2, vec![Axioms::A]);


                let f  = Function::new(&alt_sig, &vec![i1.to_term(guideline,axioms), i2.to_term(&guideline,axioms)]);

                Term::Function(f)
            },
            Interaction::Tensor(box1,box2)=>{
                let i1 = *box1.clone();
                let i2 = *box2.clone();

                let tensor_sig: FunctionSignature = FunctionSignature::new("tensor".to_string(), 2, axioms.clone());


                let f  = Function::new(&tensor_sig, &vec![i1.to_term(&guideline,axioms), i2.to_term(&guideline,axioms)]);

                Term::Function(f)
            }
        }
    }

}


