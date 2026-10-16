use std::cmp::Ordering;
use std::collections::HashSet;
use std::fmt;
use crate::interactions::syntax::action::Action;
use crate::interactions::syntax::operators::Operator;




#[derive(Clone, PartialEq, Debug, Eq, PartialOrd, Ord, Hash)]
pub enum LoopKind {
    PInterleaving  = 1,
    WWeakSeq       = 2,
    HHeadFirstWS   = 3,
    SStrictSeq     = 4
}



#[derive(Clone, PartialEq, Debug, Eq, Hash, PartialOrd, Ord)]
pub enum Interaction {
    Empty,
    Action(Action),
    Vp(Action,Action), //the order is meant to be emission-reception
    Seq(Box<Interaction>,Box<Interaction>),
    Alt(Box<Interaction>,Box<Interaction>),
    Par(Box<Interaction>,Box<Interaction>),
    CoReg(Vec<usize>,Box<Interaction>,Box<Interaction>),
   // LoopS(Box<Interaction>),
    Loop(LoopKind,Box<Interaction>),
    Tensor(Box<Interaction>,Box<Interaction>),
}



impl Interaction {

    pub fn head_symbol(&self)->Option<Operator>{
        match self.clone() {
            Interaction::Empty => None,
            Interaction::Action(_act)=>{None},
            Interaction::Vp(_act1, _act2)=>{None},
            Interaction::Loop(kind,_)=> {
                Some(Operator::Loop(kind))
            },
            Interaction::Seq(_,_)=>{
                Some(Operator::Seq)
            },
            Interaction::Par(_,_)=>{
                Some(Operator::Par)
            },
            Interaction::Alt(_,_)=>{
                Some(Operator::Alt)
            },
            Interaction::Tensor(_,_)=>{
                Some(Operator::Tensor)
            },
            Interaction::CoReg(lfs,_,_)=>{
                Some(Operator::Coreg(lfs))
            }
        }
    }

    pub fn common_gates(i:&Interaction,j:&Interaction)->HashSet<usize>{
        let gates_i = i.gates();
        let gates_j = j.gates();
        let common_gates = gates_i.intersection(&gates_j);

        let mut ret: HashSet<usize> = HashSet::new();

        for g in common_gates{
            ret.insert(*g);
        }

        ret
    }



    pub fn get_lifelines(&self) -> HashSet<usize> {
        match self.clone(){
            Interaction::Empty =>{
                HashSet::new()
            },
            Interaction::Action(act) => {
                HashSet::from([act.lf_id])
            },
            Interaction::Vp(act1,act2) => {
                HashSet::from([act1.lf_id,act2.lf_id])
            },
            Interaction::Loop(_,box1) =>{
                let i1 = *box1;
                i1.get_lifelines()
            }
            Interaction::Seq(box1,box2)
            | Interaction::Alt(box1,box2)
            | Interaction::Par(box1,box2)
            | Interaction::Tensor(box1,box2)
            | Interaction::CoReg(_,box1,box2)=> {
                let i1 = *box1;
                let i2 = *box2;

                let mut ret = HashSet::new();

                for lf in i1.get_lifelines(){
                    ret.insert(lf);
                }

                for lf in i2.get_lifelines(){
                    ret.insert(lf);
                }
                ret
            }
        }
    }

    pub fn project(&self,lifelines: &HashSet<usize>) -> Interaction {
        match self.clone() {
            Interaction::Empty => Interaction::Empty,
            Interaction::Action(act)=>{
                if lifelines.contains(&act.lf_id){
                    Interaction::Action(act)
                }
                else{
                    Interaction::Empty
                }
            },
            Interaction::Vp(act1,act2) => {
                if lifelines.contains(&act1.lf_id) && lifelines.contains(&act2.lf_id){
                    Interaction::Vp(act1,act2)
                }
                else if lifelines.contains(&act1.lf_id) && !lifelines.contains(&act2.lf_id){
                    Interaction::Action(act1)
                }
                else if !lifelines.contains(&act1.lf_id) && lifelines.contains(&act2.lf_id){
                    Interaction::Action(act2)
                }
                else{
                    Interaction::Empty
                }
            },
            Interaction::Loop(kind,box1) =>{
                let i = *box1;
                Interaction::wrap_loop(&kind,&i.project(lifelines))
            }
            Interaction::Seq(box1,box2)
            | Interaction::Alt(box1,box2)
            | Interaction::Par(box1,box2)
            | Interaction::Tensor(box1,box2)
            | Interaction::CoReg(_,box1,box2)=> {
                let i = *box1;
                let j = *box2;

                Interaction::wrap_binary(&i.project(lifelines),&j.project(lifelines), &self.head_symbol().unwrap())
            }
        }

    }


    pub fn free_gates(&self) ->HashSet<usize>{

        match self.clone() {
            Interaction::Action(a)=>{
                if let Some(gate_id) = a.gate_id{
                    HashSet::from([gate_id])
                }
                else{
                    HashSet::new()
                }

            },
            Interaction::Loop(_,box1)=>{
                let i = *box1;
                i.free_gates()
            },
            Interaction::Seq(box1, box2)
            |Interaction::Par(box1, box2)
            |Interaction::Alt(box1, box2)
            |Interaction::Tensor(box1, box2)
            |Interaction::CoReg(_,box1,box2)=>{
                let i = *box1;
                let j = *box2;
                let mut set1 = i.free_gates();

                set1.extend(j.free_gates());

                set1
            },
            _ => HashSet::new()
        }
    }
    pub fn gates(&self)->HashSet<usize>{

        match self.clone() {
            Interaction::Empty =>{
                HashSet::new()
            },
            Interaction::Action(a)=>{
                if let Some(gate_id) = a.gate_id{
                    HashSet::from([gate_id])
                }
                else{
                    HashSet::new()
                }

            },
            Interaction::Vp(a,b)=>{
              match (a.gate_id,b.gate_id){
                  (Some(g1),Some(g2))=>{
                      HashSet::from([g1,g2])
                  },
                  (Some(g1),None)=>{
                      HashSet::from([g1])
                  },
                  (None,Some(g2))=>{
                      HashSet::from([g2])
                  },
                  (None,None)=>{
                      HashSet::new()
                  }
              }
            },
            Interaction::Loop(_,box1)=>{
                let i = *box1;
                i.gates()
            },
            Interaction::Seq(box1, box2)
            |Interaction::Par(box1, box2)
            |Interaction::Alt(box1, box2)
            |Interaction::Tensor(box1, box2)
            |Interaction::CoReg(_,box1,box2)=>{
                let i = *box1;
                let j = *box2;
                let mut set1 = i.gates();

                set1.extend(j.gates());

                set1
            }
        }
    }

    pub fn gates_vec(&self)->Vec<usize>{
        let set = self.gates();
        let mut v: Vec<_> = set.iter().copied().collect();
        v.sort();
        v
    }
    fn vec_cmp(v1: &Vec<usize>, v2: &Vec<usize>) -> Ordering {
        if v1.len()==0 && v2.len()==0 {
            return Ordering::Equal;
        }
        else if v1.len()==0 && v2.len()!=0 {
            return Ordering::Less;
        }
        else if v1.len()!=0 && v2.len()==0 {
            return Ordering::Greater;
        }
        else if v1[0] < v2[0] {
            return Ordering::Less;
        }
        else if v1[0] > v2[0] {
            return Ordering::Greater;
        }
        else{
            Self::vec_cmp(&Vec::from(&v1[1..]), &Vec::from(&v2[1..]))
        }
    }
    pub fn gates_cmp(&self,j:&Interaction)->Ordering{
        let gates_i = self.gates_vec();
        let gates_j = j.gates_vec();

        Self::vec_cmp(&gates_i, &gates_j)
    }


    pub fn size(&self)->usize{
        match self{
            Interaction::Empty
            | Interaction::Action(_)
            | Interaction::Vp(_,_) =>{
                1
            },
            Interaction::Loop(_,box1)=>{
                let i1 = *box1.clone();
                1+ i1.size()
            },
            Interaction::Seq(box1,box2)
            |Interaction::Par(box1,box2)
            |Interaction::Alt(box1,box2)
            |Interaction::Tensor(box1,box2)
            |Interaction::CoReg(_,box1,box2)=>{
                let i1 = *box1.clone();
                let i2 = *box2.clone();

                1 + i1.size() + i2.size()
            }
        }
    }


    pub fn express_empty(&self) -> bool {
        match *self {
            Interaction::Empty => {
                true
            },
            Interaction::Action(_) => {
                false
            },
            Interaction::Seq(ref i1, ref i2) => {
                i1.express_empty() && i2.express_empty()
            },
            Interaction::Par(ref i1, ref i2) => {
                i1.express_empty() && i2.express_empty()
            },
            Interaction::Alt(ref i1, ref i2) => {
                i1.express_empty() || i2.express_empty()
            },
            Interaction::CoReg(_,ref i1, ref i2) => {
                i1.express_empty() && i2.express_empty()
            }
            Interaction::Loop(_,_) => {
                true
            },
            Interaction::Tensor(ref i1, ref i2) => {
                i1.express_empty() && i2.express_empty()
            },
            _ => {
                panic!("non-conform interaction");
            }
        }
    }


    pub fn clean_gates(&self)->Interaction{
        match self{
            Interaction::Empty =>{ self.clone() }
            Interaction::Action(a)=>{
                Interaction::Action(a.clean_gate())
            },
            Interaction::Vp(a,b)=>{
                Interaction::Vp(a.clean_gate(),b.clean_gate())
            },
            Interaction::Loop(kind,box1)=>{
                let i1 = *box1.clone();
                Interaction::wrap_loop(kind,&i1.clean_gates())
            },
            Interaction::Seq(box1,box2)
            |Interaction::Par(box1,box2)
            |Interaction::Alt(box1,box2)
            |Interaction::Tensor(box1,box2)
            |Interaction::CoReg(_,box1,box2)=>{
                let i1 = *box1.clone();
                let i2 = *box2.clone();
                Interaction::wrap_binary(&i1.clean_gates(),&i2.clean_gates(),&self.head_symbol().unwrap())
            }
        }
    }

}


impl fmt::Display for Interaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {


        let result = match self.clone() {
            Interaction::Empty =>{
                "Empty".to_string()
            },
            Interaction::Action(a)=>{
                format!("{}", a)
            },
            Interaction::Vp(a,b)=>{
                format!("Vp({},{})", a,b)
            },
            Interaction::Loop(kind,v)=>{
                match kind {
                    LoopKind::PInterleaving =>  format!("LoopP({})", *v),
                    LoopKind::WWeakSeq =>  format!("LoopW({})", *v),
                    LoopKind::HHeadFirstWS =>  format!("LoopH({})", *v),
                    LoopKind::SStrictSeq =>  format!("LoopS({})", *v),
                }
            },
            Interaction::Seq(l,r)=>{
                format!("Seq({},{})", *l, *r)
            },
            Interaction::Par(l,r)=>{
                format!("Par({},{})", *l, *r)
            },
            Interaction::CoReg(lfs,l,r)=>{
                
                let lfs_as_string = lfs.iter().map(|lf| format!("{}",lf)).collect::<Vec<String>>().join(",");
                let lfs_as_string = format!("[{}]",lfs_as_string);
                
                format!("CoReg({},{},{})", lfs_as_string, *l, *r)
            }
            Interaction::Alt(l,r)=>{
                format!("Alt({},{})", *l, *r)
            },
            Interaction::Tensor(l,r)=>{
                format!("Tensor({},{})", *l, *r)
            }
        };

        write!(f, "{}", result.trim_end())
    }
}
