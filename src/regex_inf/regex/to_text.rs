use std::fs::File;
use std::io::Write;
use itertools::Itertools;
use crate::automata::automaton::automaton::Automaton;
use crate::regex_inf::regex::regex::Regex;


impl ToString for Regex<String> {
    fn to_string(&self) -> String{
       // self.to_text(Some('.'))
        self.pretty_print_for_reports(Some('.'))
    }
}

impl Regex<String>{
    pub fn to_text(&self,separator: Option<char>)->String{
        //let mut ret_string: String = String::from("");

        match self {
            Regex::Empty => "".to_string(),
            Regex::Epsilon => "€".to_string(),
            Regex::Literal(a)=> a.clone(),
            Regex::Concat(label) => {
                let mut ret_string = String::from("");
                let mut ct = label.len();

                ret_string.push('(');
                for elem in label{
                    ret_string.push_str(elem.clone().to_text(separator).as_str());
                    if let Some(sep) = separator{
                        if ct>1{
                            ret_string.push(sep);
                        }
                    }
                    ct-=1;

                }
                ret_string.push(')');



                ret_string
            },
            Regex::Or(set)=> {
                let mut ret_string = String::from("");

                if set.len()==1{
                    ret_string.push_str(set.iter().collect_vec()[0].to_text(separator).as_str());
                }else if set.len()>1{
                    let mut ct = set.len();
                    ret_string.push('(');
                    for el in set{
                        // ret_string.push_str(el.to_text(separator).as_str());

                        //ret_string.push_str(format!("({})",el.to_text(separator)).as_str());
                        ret_string.push_str(el.to_text(separator).as_str());
                        if ct>1{
                            ret_string.push('|');
                        }
                        ct-=1;
                    }
                    ret_string.push(')');
                }

                ret_string
            },
            Regex::Int(set)=>{
                let mut ret_string = String::from("");

                if set.len()==1{
                    ret_string.push_str(set.iter().collect_vec()[0].to_text(separator).as_str());
                }else if set.len()>1{
                    let mut ct = set.len();
                    ret_string.push('(');
                    for el in set{
                        //ret_string.push_str(el.to_text(separator).as_str());
                        // ret_string.push_str(format!("({})",el.to_text(separator)).as_str());
                        ret_string.push_str(el.to_text(separator).as_str());
                        if ct>1{
                            ret_string.push('&');
                        }
                        ct-=1;
                    }
                    ret_string.push(')');
                }

                ret_string
            },
            Regex::Kleene(label) => {
                let internal_regex = *label.clone();
                format!("{}*",internal_regex.to_text(separator))},
            Regex::Plus(label)=> {
                let internal_regex = *label.clone();
                format!("{}+",internal_regex.to_text(separator))},
            Regex::Qm(label) => {
                let internal_regex = *label.clone();
                format!("({})?",internal_regex.to_text(separator))},
            _ => "".to_string() // for SRC and SNK
        }

    }


    pub fn to_file(&self,parent_dir: &str,name: &str)->std::io::Result<()>{
        let path_str = format!("{}/{}.txt",parent_dir,name);
        let mut file = File::create(&path_str)?;
        file.write_all(self.to_text(Some('.')).as_bytes())
    }
}


impl Regex<String>{
    pub fn pretty_print_for_reports(&self,separator: Option<char>)->String{
        match self{
            Regex::Empty => "".to_string(),
            Regex::Epsilon => "ε".to_string(),
            Regex::Literal(s)=> {
                if let Ok(pretty_action) = Automaton::simple_parse_action_from_string(s){
                    pretty_action
                }
                else{
                    s.to_string()
                }
            },
            Regex::Concat(label) => {
                let mut ret_string = String::from("");
                let mut ct = label.len();

                ret_string.push('(');
                for elem in label{
                    ret_string.push_str(elem.clone().pretty_print_for_reports(separator).as_str());
                    if let Some(sep) = separator{
                        if ct>1{
                            ret_string.push(sep);
                        }
                    }
                    ct-=1;

                }
                ret_string.push(')');



                ret_string
            },
            Regex::Or(set)=> {
                let mut ret_string = String::from("");

                if set.len()==1{
                    ret_string.push_str(set.iter().collect_vec()[0].pretty_print_for_reports(separator).as_str());
                }else if set.len()>1{
                    let mut ct = set.len();
                    ret_string.push('(');
                    for el in set{
                        // ret_string.push_str(el.pretty_print_for_reports(separator).as_str());

                        //ret_string.push_str(format!("({})",el.pretty_print_for_reports(separator)).as_str());
                        ret_string.push_str(el.pretty_print_for_reports(separator).as_str());
                        if ct>1{
                            ret_string.push('|');
                        }
                        ct-=1;
                    }
                    ret_string.push(')');
                }

                ret_string
            },
            Regex::Int(set)=>{
                let mut ret_string = String::from("");

                if set.len()==1{
                    ret_string.push_str(set.iter().collect_vec()[0].pretty_print_for_reports(separator).as_str());
                }else if set.len()>1{
                    let mut ct = set.len();
                    ret_string.push('(');
                    for el in set{
                        //ret_string.push_str(el.pretty_print_for_reports(separator).as_str());
                        // ret_string.push_str(format!("({})",el.pretty_print_for_reports(separator)).as_str());
                        ret_string.push_str(el.pretty_print_for_reports(separator).as_str());
                        if ct>1{
                            ret_string.push('&');
                        }
                        ct-=1;
                    }
                    ret_string.push(')');
                }

                ret_string
            },
            Regex::Kleene(label) => {
                let internal_regex = *label.clone();
                format!("{}*",internal_regex.pretty_print_for_reports(separator))},
            Regex::Plus(label)=> {
                let internal_regex = *label.clone();
                format!("{}+",internal_regex.pretty_print_for_reports(separator))},
            Regex::Qm(label) => {
                let internal_regex = *label.clone();
                format!("({})?",internal_regex.pretty_print_for_reports(separator))},
            _ => "".to_string() // for SRC and SNK
        }
    }
}