use itertools::Itertools;
use crate::global_counter::counter::fresh_number;
use crate::interactions::composition::error::CompositionError;
use crate::interactions::syntax::action::Action;
use crate::interactions::syntax::general_context::GeneralContext;
use crate::interactions::syntax::interaction::Interaction;
use crate::terms::function::Axioms;

pub struct Tag{
    pub a1: Action,
    pub a2: Action,
    pub gate:usize,
}
impl Tag{
    pub fn new(a1: &Action, a2: &Action, gate:usize) -> Tag{
        Tag{a1:a1.clone(), a2: a2.clone(), gate}
    }
}

type Tagging = Vec<Tag>;
impl Interaction {

    pub fn tag_and_compose_pairwise(i:&Interaction, j:&Interaction, timeout: &Option<f64>)->Result<Interaction,CompositionError>{
        let tagging = Self::extract_taggings(i,j);
        let tagged = Self::apply_tagging(i,j,&tagging);

        Self::compose(&tagged[0],&tagged[1],/*false,false,true,timeout.clone(),*/ &Axioms::acu(),false,None)
    }
    
    pub fn tagging(ints: &Vec<Interaction>)->Vec<Interaction>{
        todo!()
    }
    
    
    pub fn tagging_pairwise(i: &Interaction, j:&Interaction) -> Vec<Interaction> {
        let tagging = Self::extract_taggings(i,j);


        Self::apply_tagging(i,j,&tagging)
    }

    pub fn apply_tagging(i: &Interaction,j: &Interaction,tagging: &Tagging) -> Vec<Interaction> {
        let mut i_tagged = i.clone();
        let mut j_tagged = j.clone();


        for tag in tagging.iter(){
            i_tagged = i_tagged.apply_tag(&tag.a1,tag.gate);
            j_tagged = j_tagged.apply_tag(&tag.a2,tag.gate);
        }

        vec![i_tagged,j_tagged]
    }
    pub fn apply_tag (&self,action: &Action, gate: usize)->Interaction {

        match self.clone() {
            Interaction::Empty | Interaction::Vp(_,_) =>{
                self.clone()
            },
            Interaction::Action(act) => {
                if act==*action{
                    Interaction::Action(act.set_gate(gate))
                }
                else{
                    self.clone()
                }
            },
            Interaction::Loop(kind,box1)=>{
                let i1 = *box1;

                Interaction::wrap_loop(&kind.clone(),&i1.apply_tag(action, gate))
            },
            Interaction::Seq(box1,box2)
            |Interaction::Par(box1,box2)
            |Interaction::Alt(box1,box2)
            |Interaction::Tensor(box1,box2)
            |Interaction::CoReg(_,box1,box2)=>{
                let i1 = *box1;
                let i2 = *box2;

                Interaction::wrap_binary(&i1.apply_tag(action,gate),&i2.apply_tag(action,gate),&self.head_symbol().unwrap())
            }
        }
    }


    pub fn extract_taggings(i: &Interaction, j:&Interaction)->Tagging{

        let mut tagging = Vec::new();

        let actions_i = i.get_free_actions();
        let actions_j = j.get_free_actions();

        for ai in actions_i.iter(){
            for aj in actions_j.iter(){
                if Action::are_complementary(ai,aj){
                    let fresh_gate = fresh_number();
                    tagging.push(Tag::new(ai,aj,fresh_gate));
                }
            }
        }


        tagging
    }

}



impl Action{

    pub fn set_gate(&self,gate:usize)->Action{
        Action::new_with_id(self.lf_id,self.ms_id,self.action_type.clone(),gate)
    }
    pub fn are_complementary(a1: &Action, a2: &Action) -> bool{
        (a1.lf_id != a2.lf_id)
        && (a1.ms_id == a2.ms_id)
        && (a1.action_type != a2.action_type)
        && (a1.gate_id == a2.gate_id)
    }


}