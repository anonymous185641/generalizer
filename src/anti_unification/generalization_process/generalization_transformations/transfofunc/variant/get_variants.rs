use std::collections::BTreeMap;
use std::process::Command;
use std::str::Chars;
use crate::anti_unification::generalization_process::generalization_transformations::transfofunc::variant::variant::Variant;
use crate::terms::function::{Function, Signature};
use crate::terms::parsing::term_parser::{parse_term, parse_variable};
use crate::terms::substitution::substitution::Substitution;
use crate::terms::substitution::variable::Variable;
use crate::terms::term::Term;




impl Variant {
    pub fn get_variants(t: &Term,signature: &Signature,additional_variables: &Vec<Variable>,loop_alt:bool)->Vec<Variant> {
        let mut variants = vec![];

        let t_stringed = t.to_string();
        //println!("t_stringed: {}",t_stringed);

        let v = Variable::fresh_variable();
        let maude_file = if !loop_alt{
            Self::generate_maude_file_interaction(&additional_variables)
        }else{
            Self::generate_maude_file_interaction_loop_alt(&additional_variables)
        };
        let script = Self::generate_python_script_interaction(&maude_file);

        //println!("in");

        let output = if cfg!(target_os = "linux") || cfg!(target_os = "macos") {
             Command::new("python3")
                .arg("-c")
                .arg(script)
                .arg(t_stringed)
                .output()
                .expect("Failed to execute command")
        } else if cfg!(target_os = "windows") {
            Command::new("py")
                .arg("-c")
                .arg(script)
                .arg(t_stringed)
                .output()
                .expect("Failed to execute command")
        }else{
            panic!("OS not supported");
        };
        /*
        let output = Command::new("python3")
            .arg("-c")
            .arg(script)
            .arg(t_stringed)
            .output()
            .expect("Failed to execute command");

         */
        //println!("out");
        if !output.status.success() {
            eprintln!(
                "Python error: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }

        let stdout = String::from_utf8_lossy(&output.stdout);

        let strings: Vec<String> = stdout
            .lines()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        //println!("strings: {:?}",strings);
        //println!("\n");
        //println!("strings: {:?}",strings);
       for s in strings.iter() {
           //println!("{:?}",s);
           match  Variant::parse_variant_line(s,signature){
               Ok(v) => variants.push(v),
               Err(e) => println!("Error parsing variant: {}",e),
           }
       }

        variants
    }

    pub fn parse_variant_line(line:&String, signature: &Signature)->Result<Variant,String>{
        //We eliminate the texts ":[Interaction]"
        let line_cleaned = line.replace(":[Interaction]","").replace(" ","");
        //println!("line_cleaned: {}",line_cleaned);
        let (term_str,subs_str) = line_cleaned.split_once("+++").unwrap();
       // println!("term_str: {} subs_str: {}",term_str,subs_str);
        //Parsing of the term
        //println!("term_str: {}",term_str);
        let t= parse_term(signature,&term_str.to_string())?;
        //println!("t: {:?}",t);
        //println!("fff {:?}",t);
        //Parsing of the substitution
        let subs = Self::special_parse_substitution(subs_str, signature)?;

        Ok(Variant::new(t,subs))
    }



    pub fn special_parse_substitution(subs_string: &str, signature: &Signature)->Result<Substitution,String>{
        let mut substitution = Substitution::new();

        //Here's the problem
        //let bindings  = subs_string.split(',').collect::<Vec<&str>>();
        let bindings = Self::split_top_level_all(subs_string);

        for binding in bindings{
            let (lhs,rhs) = binding.split_once('=').unwrap();

            let left_var = parse_variable(lhs)?;
            //println!("left var OK t: {}",left_var);

            let right_term = parse_term(signature,&rhs.to_string())?;

            substitution.insert(&left_var,&right_term);

        }

        Ok(substitution)
    }

    fn split_top_level_all(s: &str) -> Vec<&str> {
        let mut depth = 0;
        let mut start = 0;
        let mut result = Vec::new();

        for (i, c) in s.char_indices() {
            match c {
                '(' => depth += 1,
                ')' => depth -= 1,
                ',' if depth == 0 => {
                    result.push(s[start..i].trim());
                    start = i + 1;
                }
                _ => {}
            }
        }

        result.push(s[start..].trim());
        result
    }
}



















