use crate::interactions::syntax::general_context::GeneralContext;
use crate::interactions::syntax::interaction::Interaction;
use crate::interactions::syntax::traces::sync_trace::SynchronousTrace;

impl Interaction{

    pub fn sync_trace_filter_message(trace: &SynchronousTrace,general_context: &GeneralContext,msg_name: &str) -> SynchronousTrace{

        let mut return_trace = SynchronousTrace::new();

        for vp in trace.iter(){
            if Some(vp.msg_id) == general_context.get_ms_id(msg_name){
                continue;
            }
            else{
                return_trace.push(vp.clone());
            }

        }

        return_trace
    }

    pub fn sync_trace_filter_messages(trace: &SynchronousTrace,general_context: &GeneralContext,msg_name: &Vec<String>) -> SynchronousTrace{
        let mut return_trace = trace.clone();

        for msg in msg_name.iter(){
            return_trace = Self::sync_trace_filter_message(&return_trace,general_context,msg);
        }
         return_trace
    }

    pub fn sync_trace_filter_lf(trace: &SynchronousTrace,general_context: &GeneralContext,lf_name: &str) -> SynchronousTrace{

        let mut return_trace = SynchronousTrace::new();

        for vp in trace.iter(){
            if Some(vp.lf_emission_id) == general_context.get_lf_id(lf_name)
            || Some(vp.lf_reception_id) == general_context.get_lf_id(lf_name){
                continue;
            }
            else{
                return_trace.push(vp.clone());
            }

        }

        return_trace
    }
    pub fn sync_trace_filter_lfs(trace: &SynchronousTrace,general_context: &GeneralContext,lf_name: &Vec<String>) -> SynchronousTrace{
        let mut return_trace = trace.clone();
        for lf in lf_name.iter(){
            return_trace = Self::sync_trace_filter_lf(&return_trace,general_context,lf);
        }
        return_trace
    }

}