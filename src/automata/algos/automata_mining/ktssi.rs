use std::collections::HashSet;
use itertools::Itertools;
use petgraph::graph::{DiGraph, NodeIndex};
use crate::automata::automaton::automaton::Automaton;
use crate::regex_inf::sample::sample::Sample;

pub struct KTSSI{}


impl KTSSI {
    pub fn ktssl_tuple(k: usize,sample: &Sample)-> (HashSet<String>,HashSet<String>,HashSet<String>,HashSet<String>){
        //returns a tuple of the form (Alphabet, Initial_segments, Final_segments, allowed_Segments, Forbidden_segments)
        let alphabet = HashSet::from_iter(sample.alphabet.clone());
        let mut initial_segments: HashSet<String> = HashSet::new();
        let mut final_segments: HashSet<String> = HashSet::new();
        let mut allowed_segments: HashSet<String> = HashSet::new();



        if k ==1 {
            return  (alphabet,initial_segments,final_segments,allowed_segments);
        }

        //Initial segments and finial segments computation

        for string in &sample.sample{
            if sample.separator == None{
                if string.len()<k-1 {
                    initial_segments.insert(string.clone());
                    final_segments.insert(string.clone());
                }
                else{
                    let init_seg = Self::first_k_letters(k-1,&string,sample.separator);
                    let final_seg = Self::last_k_letters(k-1,&string,sample.separator);
                    if !init_seg.is_empty(){
                        initial_segments.insert(init_seg);
                    }
                    if !final_seg.is_empty(){
                        final_segments.insert(final_seg);
                    }

                }
            }else{
                let mut separated_iterator = string.split_terminator(sample.separator.unwrap()).collect_vec();
                if separated_iterator.len()<k-1 {
                    initial_segments.insert(string.clone());
                    final_segments.insert(string.clone());
                }
                else{
                    let init_seg = Self::first_k_letters(k-1,&string,sample.separator);
                    let final_seg = Self::last_k_letters(k-1,&string,sample.separator);
                    if !init_seg.is_empty(){
                        initial_segments.insert(init_seg);
                    }
                    if !final_seg.is_empty(){
                        final_segments.insert(final_seg);
                    }

                }
            }

        }


        allowed_segments = sample.get_kgrams(k);


        (alphabet,initial_segments,final_segments,allowed_segments)
    }

    pub fn ktssi(k: usize,sample:&Sample)->Automaton{// OK
        let mut output_automaton = Automaton::new();
        //output_automaton.aut_type = AutomatonType::DFA;

        let (alphabet, initial_segments,final_segments,allowed_segments) = Self::ktssl_tuple(k,sample);

        if k==1 {
            return output_automaton;
        }

        //Initial state
        let q0 = output_automaton.get_or_create_node(&"".to_string());
        output_automaton.initial_states.push(q0);
        //Handling initial segments nodes and edges
        for init_seg in initial_segments{
            let mut init_seg_lenght: usize = 0;
            if sample.separator==None{
                init_seg_lenght = init_seg.len();
            }
            else{
                init_seg_lenght = init_seg.split_terminator(sample.separator.unwrap()).collect_vec().len();
            }
            for j in 1..=init_seg_lenght{
                let n1 = Self::first_k_letters(j-1,&init_seg,sample.separator);
                let a_j = Self::k_th_letter(j,&init_seg,sample.separator);
                let n2 = Self::first_k_letters(j,&init_seg,sample.separator);

                let n1_id = output_automaton.get_or_create_node(&n1);
                let n2_id = output_automaton.get_or_create_node(&n2);
                output_automaton.graph.update_edge(n1_id,n2_id,a_j);

            }
        }

        //Handling the remaining
        for allow_seg in allowed_segments{
            let n1 = Self::first_k_letters(k-1,&allow_seg,sample.separator);
            let a_k = Self::k_th_letter(k,&allow_seg,sample.separator);
            let n2 = Self::last_k_letters(k-1,&allow_seg,sample.separator);

            let n1_id = output_automaton.get_or_create_node(&n1);
            let n2_id = output_automaton.get_or_create_node(&n2);
            output_automaton.graph.update_edge(n1_id,n2_id,a_k);
        }

        //Final states
        for label in final_segments{
            if let Some(node_id) = output_automaton.get_node(&label){
                output_automaton.final_states.push(node_id);
            }
        }


        output_automaton
    }

    pub fn first_k_letters(k: usize, string: &String,separator: Option<char>)->String{
        let mut result = String::from("");

        if k==0 {
            return result;
        }

        if separator==None{
            if string.len()<k{
                return result;
            }
            let mut separated_iterator = string.chars().collect_vec();
            for j in 0..k{
                result.push(separated_iterator[j]);
            }
        }
        else{
            let mut separated_iterator = string.split_terminator(separator.unwrap()).collect_vec();

            if separated_iterator.len()<k{
                return result;
            }
            //***************
            for j in 0..k{
                result.push_str(separated_iterator[j]);
                if j<k-1{
                    result.push(separator.unwrap());
                }
            }
        }

        result
    }
    pub fn k_th_letter(k: usize,string: &String,separator: Option<char>)->String{
        if k==0 || string.is_empty(){
            return String::from("");
        }

        if separator==None{

            let mut separated_iterator = string.chars().collect_vec();
            if string.len()<k {
                return String::from("");
            }
            else {
                return String::from(separated_iterator[k-1]);
            }

        }
        else{
            let mut separated_iterator = string.split_terminator(separator.unwrap()).collect_vec();

            if separated_iterator.len()<k{
                return String::from("");
            }
            else{
                return String::from(separated_iterator[k-1]);
            }

        }
    }
    pub fn last_k_letters(k: usize, string: &String,separator: Option<char>)->String{
        let mut result = String::from("");

        if k==0 {
            return result;
        }

        if separator==None{
            let mut separated_iterator = string.chars().collect_vec();
            let size = separated_iterator.clone().len();

            if size>=k{
                for j in size-k..size{
                    result.push(separated_iterator[j]);
                }
            }

        }
        else{
            let mut separated_iterator = string.split_terminator(separator.unwrap()).collect_vec();
            let size = separated_iterator.clone().len();
            if size>=k{
                for j in size-k..size{
                    result.push_str(separated_iterator[j]);
                    if j<size-1{
                        result.push(separator.unwrap());
                    }
                }
            }

        }

        result
    }

}