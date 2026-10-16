use std::fs;

pub fn print_file(file: &str,alpuente: bool) {
    match fs::read_to_string(&file){
        Ok(s) => {
            println!(
                "\n========================================\n\
                    🦀  Input \n\
                    ========================================\n"
            );
            println!("{}", s);


            println!(
                "\n========================================\n\
                    🦀  Algorithm \n\
                    ========================================\n"
            );

            if alpuente{
                println!("Generalization using the rules of Alpuente et al.");
                println!("CAVEAT: The rules of Alpuente et al. as implemented compute only linear generalisations modulo unit, and is incomplete");
            }
            else {
                println!("Generalization using custom rules inspired from the work of Alpuente et al.");
            }
            println!(
                "\n========================================\n\
                    🦀  Output \n\
                    ========================================\n"
            );
        }
        Err(e) => {
            println!("Error reading file {}", e);
        }
    }
}

pub fn print_file_petri_to_regex(file:&str){
    match fs::read_to_string(&file){
        Ok(s) => {
            println!(
                "\n=======================================================================\n\
                    Input Petri Net {} Successfully Parsed \n\
                    =======================================================================\n",file
            );
            println!("{}", s);


            println!(
                "\n=======================================================================\n\
                    Starting the transformation to regular expression of {} \n\
                    =======================================================================\n",file
            );
        }
        Err(e) => {
            println!("Error reading file {}", e);
        }
    }
}

pub fn print_automaton_file(file:&str){
    match fs::read_to_string(&file){
        Ok(s) => {
            println!(
                "\n=======================================================================\n\
                    Input Automaton {} Successfully Parsed \n\
                    =======================================================================\n",file
            );
            println!("{}", s);


            println!(
                "\n=======================================================================\n\
                    Starting the transformation to regular expression of {} \n\
                    =======================================================================\n",file
            );
        }
        Err(e) => {
            println!("Error reading file {}", e);
        }
    }
}