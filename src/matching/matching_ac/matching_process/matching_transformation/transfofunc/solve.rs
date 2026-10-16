use crate::matching::matching_ac::matching_configuration::matching_configuration::MatchingConfiguration;
use crate::terms::term::Term;

pub fn can_apply_m_solve(config : &MatchingConfiguration) -> bool {

    let head_problem = config.unsolved_set[0].clone();

    if head_problem.0.is_variable(){
        return true;
    }

    false
}

pub fn tranfo_m_solve(config : &MatchingConfiguration) -> Vec<MatchingConfiguration> {
    let mut new_unsolved_set = config.unsolved_set.clone();
    let new_partial_solution = config.partly_solved_set.clone();
    let mut new_solution = config.solved_set.clone();
    
    let head_problem = new_unsolved_set.remove(0);
    
    match head_problem.0 {
        Term::Variable(v) => {
            new_solution.push((v,head_problem.1));
        },
        _ => panic!("head problem is not of the form x = s. Impossible since Solve is applicable")
    }
    
    vec![MatchingConfiguration {
        unsolved_set: new_unsolved_set,
        partly_solved_set: new_partial_solution,
        solved_set: new_solution
    }]
}





















