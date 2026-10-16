use crate::anti_unification::configuration::aut::AUT;
use crate::matching::matching_ac::matching_configuration::matching_configuration::MatchingConfiguration;
use crate::terms::substitution::variable::Variable;
use crate::terms::term::Term;

pub fn can_apply_m_decompose(config : &MatchingConfiguration) -> bool {

    let head_problem = config.unsolved_set[0].clone();

    //To complete with check on no equations of the head symbol
    if (head_problem.0.head()==head_problem.1.head())
        && !head_problem.0.is_constant_symbol() //To avoid infinite loop on constant symbols 
        && head_problem.0.is_head_function_free()  
        && !head_problem.0.is_variable() //This condition is to not make the confusion with Solve
    {
        return true;
    }

    false
}

pub fn transfo_m_decompose(config : &MatchingConfiguration) -> Vec<MatchingConfiguration> {

    let mut new_unsolved_set = config.unsolved_set.clone();
    let new_partial_solution = config.partly_solved_set.clone();
    let new_solution = config.solved_set.clone();


    let head_problem = new_unsolved_set.remove(0);

    match (head_problem.0,head_problem.1) {
        (Term::Function(f1),Term::Function(f2)) => {


            for (t1, t2) in f1.args.iter().zip(f2.args.iter()) {
                let x = Variable::fresh_variable();
                new_unsolved_set.insert(0, (t1.clone(),t2.clone()));
            }

        },
        _ => panic!("Error: decompose only works on terms with same head")
    }



    vec![MatchingConfiguration {
        unsolved_set: new_unsolved_set,
        partly_solved_set: new_partial_solution,
        solved_set: new_solution
    }]
}