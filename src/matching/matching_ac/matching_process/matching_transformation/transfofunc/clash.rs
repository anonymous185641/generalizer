use crate::matching::matching_ac::matching_configuration::matching_configuration::MatchingConfiguration;

pub fn can_apply_clash(config : &MatchingConfiguration) -> bool {

    let head_problem = config.unsolved_set[0].clone();

    if head_problem.0.head() != head_problem.1.head()
    && !head_problem.0.is_variable() //This condition is to not make the confusion with Solve rule
    {
        return true;
    }

    false
}


pub fn transfo_clash(config : &MatchingConfiguration) -> Vec<MatchingConfiguration> {
    vec![]
}


















