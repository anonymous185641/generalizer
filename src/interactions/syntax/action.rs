use std::fmt;

#[derive(Clone, PartialEq, Debug, Eq, Hash, PartialOrd,Ord)]
pub enum ActionType{
    Emission,
    Reception
}
#[derive(Clone, PartialEq, Debug, Eq, Hash, PartialOrd, Ord)]
pub struct Action{
    pub lf_id: usize,
    pub ms_id: usize,
    pub action_type: ActionType,
    pub gate_id: Option<usize>,
}

impl Action {
    pub fn new(lf_id: usize, ms_id: usize, action_type: ActionType/*,gateway_id: usize*/) -> Action {
        Action{lf_id, ms_id, action_type, gate_id: None}
    }
    pub fn new_with_id(lf_id: usize, ms_id: usize, action_type: ActionType,gate_id: usize) -> Action {
        Action{lf_id, ms_id, action_type, gate_id: Some(gate_id)}
    }

    pub fn new_with_id_raw(lf_id: usize, ms_id: usize, action_type: ActionType,gate_id: Option<usize>) -> Action {
        Action{lf_id, ms_id, action_type, gate_id }
    }

   pub  fn action_label(&self)->String{
        let ty = if self.action_type==ActionType::Emission{
            0
        }
        else{
            1
        };

        let gate_id = self.gate_id.unwrap_or_else(|| 0);

        format!("Action({},{},{},{})",self.lf_id,self.ms_id,ty,gate_id)
    }

    pub fn clean_gate(&self)->Action{
        Action::new(self.lf_id, self.ms_id, self.action_type.clone())
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       let  result = if let Some(g) = self.gate_id{
            if self.action_type==ActionType::Emission{
                format!("l{}!m{}[{}]", self.lf_id+1,self.ms_id+1,g)
            }
            else{
                format!("l{}?m{}[{}]", self.lf_id+1,self.ms_id+1,g)
            }

        }
        else{
            if self.action_type==ActionType::Emission{
                format!("l{}!m{}", self.lf_id+1,self.ms_id+1)
            }
            else{
                format!("l{}?m{}", self.lf_id+1,self.ms_id+1)
            }
        };

        write!(f, "{}", result.trim_end())
    }
}