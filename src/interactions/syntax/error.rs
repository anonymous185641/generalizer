



use std::fmt;



#[derive(Debug)]
pub enum CoreError {
    UnknownLifeline(usize),
    UnknownMessage(usize),
    #[allow(dead_code)]
    UnknownGate(usize)
}

impl fmt::Display for CoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CoreError::UnknownLifeline( lf_id ) => {
                return write!(f, "{}", format!("context error ; unknown lifeline : {:}", lf_id));
            },
            CoreError::UnknownMessage( ms_id ) => {
                return write!(f, "{}", format!("context error ; unknown message : {:}", ms_id));
            },
            CoreError::UnknownGate( gt_id ) => {
                return write!(f, "{}", format!("context error ; unknown gate : {:}", gt_id));
            }
        }
    }
}
