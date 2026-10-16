use std::fmt;
use std::fmt::{Debug, Formatter};

//********************************

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Position {
    Epsilon(Option<usize>),
    Left(Box<Position>),
    Right(Box<Position>),
    #[allow(dead_code)]
    Both(Box<Position>, Box<Position>)
}

impl Debug for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Position::Epsilon(sub_pos) => {
                match sub_pos {
                    None => {
                        write!(f,"o")
                    },
                    Some(sbp_idx) => {
                        write!(f,"s{:}",sbp_idx)
                    }
                }
            },
            Position::Left(ref in_self) => {
                write!(f,"1{:}",in_self)
            },
            Position::Right(ref in_self) => {
                write!(f,"2{:}",in_self)
            },
            Position::Both(ref sub1, ref sub2) => {
                write!(f,"H1{:}_2{:}H",sub1,sub2)
            }
        }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Position::Epsilon(sub_pos) => {
                match sub_pos {
                    None => {
                        write!(f,"")
                    },
                    Some(sbp_idx) => {
                        write!(f,"s{:}",sbp_idx)
                    }
                }
            },
            Position::Left(ref in_self) => {
                write!(f,"1{:}",in_self)
            },
            Position::Right(ref in_self) => {
                write!(f,"2{:}",in_self)
            },
            Position::Both(ref sub1, ref sub2) => {
                write!(f,"(1{:},2{:})",sub1,sub2)
            }
        }
    }
}





