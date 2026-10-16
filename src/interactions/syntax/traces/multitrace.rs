use crate::interactions::syntax::traces::trace::Trace;

pub type MultiTrace = Vec<Trace>;



pub fn multi_trace_length(multi_trace : &MultiTrace) -> usize {
    let mut length = 0;
    for trace in multi_trace {
        length += trace.len();
    }
    return length;
}