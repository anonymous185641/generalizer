use std::str::Chars;
use crate::terms::function::{Function, Signature};
use crate::terms::substitution::variable::Variable;
use crate::terms::term::Term;


pub fn parse_term(sig: &Signature, term_string: &String) -> Result<Term, String> {
    let mut chars = term_string.chars().peekable();
    let term = parse_from_chars(sig, &mut chars)?;
    if chars.peek().is_some() {
        return Err("Unexpected trailing characters".to_string());
    }
    Ok(term)
}



fn parse_from_chars(
    sig: &Signature,
    chars: &mut std::iter::Peekable<Chars<'_>>,
) -> Result<Term, String> {
    let name = parse_identifier(chars)?;

    if let Some('(') = chars.peek() {
        // function with arguments
        chars.next(); // consume '('

        let mut args = Vec::new();

        if let Some(')') = chars.peek() {
            chars.next(); // consume ')'
        } else {
            loop {
                let arg = parse_from_chars(sig, chars)?;
                args.push(arg);

                match chars.next() {
                    Some(',') => continue,
                    Some(')') => break,
                    Some(c) => return Err(format!("Unexpected character '{}'", c)),
                    None => return Err("Unexpected end of input".to_string()),
                }
            }
        }

        // Find function signature
        if let Some(fsig) = sig.iter().find(|s| s.name == name) {
            if fsig.arity == 2 && args.len() >= 2 {
                // Interpret:
                //
                // f(a,b,c,d)
                //
                // as:
                //
                // f(a, f(b, f(c,d)))
                //
                let mut iter = args.into_iter().rev();

                let mut result = iter
                    .next()
                    .expect("args.len() >= 2");

                for arg in iter {
                    result = Term::Function(Function {
                        signature: fsig.clone(),
                        args: vec![arg, result],
                    });
                }

                Ok(result)
            } else {
                // For functions whose arity is not 2,
                // require the exact number of arguments.
                if fsig.arity != args.len() {
                    return Err(format!(
                        "Function '{}' expects {} arguments, but {} were provided",
                        fsig.name,
                        fsig.arity,
                        args.len()
                    ));
                }

                Ok(Term::Function(Function {
                    signature: fsig.clone(),
                    args,
                }))
            }
        } else {
            Err(format!("Unknown function '{}'", name))
        }
    } else {
        // bare identifier → must be a 0-arity function
        if let Some(fsig) = sig.iter().find(|s| s.name == name) {
            if fsig.arity != 0 {
                return Err(format!(
                    "Function '{}' used without arguments but has arity {}",
                    fsig.name, fsig.arity
                ));
            }

            Ok(Term::Function(Function {
                signature: fsig.clone(),
                args: Vec::new(),
            }))
        } else {
            // Here you parse whatever is here as a variable
            let var = parse_variable(&name)?;
            Ok(Term::Variable(var))
        }
    }
}
fn parse_identifier(chars: &mut std::iter::Peekable<Chars<'_>>) -> Result<String, String> {
    let mut ident = String::new();
    while let Some(&c) = chars.peek() {
        if c.is_alphanumeric() || c == '_'|| c=='#' || c =='%' || c =='[' || c == ']' {
            ident.push(c);
            chars.next();
        } else {
            break;
        }
    }
    if ident.is_empty() {
        Err("Expected identifier".to_string())
    } else {
        Ok(ident)
    }
}

pub fn parse_variable(name: &str) -> Result<Variable, String> {
    //println!("NAME????: {}",name);
    let split = name
        .rfind(|c: char| !c.is_ascii_digit())
        .map(|i| i + 1)
        .unwrap_or(0);

    let (label, digits) = name.split_at(split);

    if label.is_empty() || digits.is_empty() {
        return Err(format!(
            "'{}' is neither a function symbol nor a variable of the form x0",
            name
        ));
    }

    let id = digits
        .parse::<usize>()
        .map_err(|_| format!("Invalid variable '{}'", name))?;

    Ok(Variable::new(label.to_string(), id))
}


