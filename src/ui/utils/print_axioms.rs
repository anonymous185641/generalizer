use crate::terms::function::Axioms;

pub fn print_axioms(axioms: &Vec<Axioms>){

    if !axioms.is_empty() {
        let mut ax_str = String::new();

        for ax in axioms {
            ax_str.push_str(ax.to_string().as_str());
        }

        println!("Using Anti-unification modulo {}\n", ax_str);
    }
    else{
        println!("Using Syntactic anti-unification (no equations)\n");
    }

}

