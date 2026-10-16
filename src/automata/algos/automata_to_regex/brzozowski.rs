use std::collections::BTreeMap;
use std::fmt;
use graphviz_rust::printer::PrinterContext;
use image::codecs::pnm::ArbitraryTuplType;
use petgraph::graph::NodeIndex;
use crate::automata::automaton::automaton::Automaton;
use crate::regex_inf::regex::regex::Regex;

impl Automaton {

    pub fn automaton_to_regex_brzozowski(aut: &Automaton) -> Regex<String> {
        aut.to_regex_brzozowski()
    }
    pub fn to_regex_brzozowski(&self) -> Regex<String> {
        let nodes: Vec<NodeIndex> = self.graph.node_indices().collect();
        let n = nodes.len();

        // Map node index to equation index
        let node_to_idx: BTreeMap<NodeIndex, usize> = nodes.iter()
            .enumerate()
            .map(|(i, &node)| (node, i))
            .collect();


        // Build adjacency matrix with regex transitions
        let mut r_matrix: Vec<Vec<Regex<String>>> = vec![vec![Regex::Empty; n]; n];

        for edge_idx in self.graph.edge_indices() {
            let (source, target) = self.graph.edge_endpoints(edge_idx).unwrap();
            let label = self.graph.edge_weight(edge_idx).unwrap().clone();

            let i = node_to_idx[&source];
            let j = node_to_idx[&target];

            let regex_label = if label.eq("Epsilon") {
                Regex::Epsilon
            } else {
                Regex::Literal(label.clone())
            };

            r_matrix[i][j] = Regex::or_vec_retain_non_empty(vec![r_matrix[i][j].clone(),regex_label]);
        }

        //Building the equations
        let mut equations = Equation::build_equation_system_from_automaton(self, &r_matrix,&nodes);
        //let mut solved_equations = vec![];
        let mut final_regex = Regex::Empty;

        println!("Before");
        Equation::print_equation(&equations);
        println!("----\n");

        //Solving the equations
        while equations.len() > 0{
            if let Some((arden_eqn,position_in_vector)) = Equation::get_arden_equation(&equations){
                //Equation::print_equation(&equations);
                println!("Arden Equation: {}",arden_eqn);
                //First we solve the equation
                let result = arden_eqn.solve_arden_equation_new().unwrap();

                final_regex = result.right_side[0].regex.clone();

                //We remove it from the pool of equations
                equations.remove(position_in_vector);
                //solved_equations.push(arden_eqn);

                // We replace the variable in the other equations
                //Equation::substitute_arden_solution_in_other_equations(&mut equations,var_id,result.clone());
                Equation::substitute_in_other_equations(&mut equations,&result.clone());
                //Equation::print_equation(&equations);
                continue;

            }

            let last_equation = equations.pop().unwrap();

            println!("Equation removed Equation {}",last_equation);

            if equations.len() == 0 {
                final_regex = last_equation.right_side[0].regex.clone();
                break;
            }
            Equation::substitute_in_other_equations(&mut equations,&last_equation);
            //println!();
            //Equation::print_equation(&equations);
        }

        //println!("After");
        //Equation::print_equation(&equations);

        //final_regex = equations.pop().unwrap().right_side[0].regex.clone();
        println!("Result {}",final_regex.to_text(Some('.')));

        final_regex
    }
}

#[derive(PartialEq,Eq,Clone)]
struct Factor{
    regex: Regex<String>,
    var: Option<usize>
}

#[derive(PartialEq,Eq,Clone)]
struct Equation{
    left_side: usize,
    right_side: Vec<Factor>,
}

impl Equation{
    fn new(left_side: usize,right_side:Vec<Factor>) -> Equation{
        Equation{left_side,right_side}
    }

    fn factorize(&mut self){
        // This function factorizes the right side of the equation
        // For example if we have and equation R1 = aR1 + bR1 + cR2, we transform it into R1 = (a+b)R1 + cR2

        for i in 0..self.right_side.len(){
            if i>self.right_side.len()-1{
                break;
            }
            for j in (i+1)..self.right_side.len(){
                if j>self.right_side.len()-1{
                    break;
                }
                if self.right_side[i].var == self.right_side[j].var{
                    self.right_side[i].regex = Regex::or_vec_retain_non_empty(vec![self.right_side[i].regex.clone(),self.right_side[j].regex.clone()]);
                    self.right_side.remove(j);
                }
            }
        }

        // We put the constant factors at the end of the equation
        let mut constant_regex = None;

        for i in 0..self.right_side.len(){
            if i>self.right_side.len()-1{
                break;
            }
            if self.right_side[i].var == None{
                constant_regex = Some(self.right_side[i].regex.clone());
                self.right_side.remove(i);
            }
        }

        if let Some(cr) = constant_regex{
            self.right_side.push(Factor{regex:cr,var:None});
        }

    }

    fn is_arden_pattern(&self)->bool{
        if self.right_side.len() != 2{
            return false;
        }

        for factor in self.right_side.iter(){
            if factor.var == Some(self.left_side){
                return true;
            }
        }
        /*
        if self.right_side[0].var ==Some(self.left_side)
        {
            return true;
        }
         */

        false
    }

    pub fn solve_arden_equation_new(&self)->Option<Equation>{
        if self.is_arden_pattern(){

            //X = AX + BY
           
            //Here A is not necessarily the first factor on the right side
            let (A,B,Y) = if self.right_side[0].var == Some(self.left_side){
                (self.right_side[0].regex.clone(),self.right_side[1].regex.clone(),self.right_side[1].var.clone())
            }else //if self.right_side[1].var == Some(self.left_side)
            {
                (self.right_side[1].regex.clone(),self.right_side[0].regex.clone(),self.right_side[0].var.clone())
            };

            //let A = self.right_side[0].regex.clone();
            //let B = self.right_side[1].regex.clone();

            let arden_result_regex = Regex::concatenate_vec(vec![Regex::kleene(&A),B]);

            //Arden formula
            let arden_result = Equation{left_side:self.left_side,right_side:vec![Factor{regex:arden_result_regex,var:Y}]};

            return Some(arden_result);
        }
        None
    }
    pub fn solve_arden_equation(&self)->Option<(Regex<String>,usize)>{
        if self.is_arden_pattern(){

            let A = self.right_side[0].regex.clone();
            let B = self.right_side[1].regex.clone();

            let arden_result = Regex::concatenate_vec(vec![Regex::kleene(&A),B]);

            return Some((arden_result,self.left_side));
        }
        None
    }
    pub fn get_arden_equation(equations: &Vec<Equation>) -> Option<(Equation,usize)>{

        for (ct,equation) in equations.iter().enumerate(){
            if equation.is_arden_pattern(){
                 return Some((equation.clone(),ct))
            }
        }

        None
    }


    pub fn substitute_arden_solution_in_other_equations(equations: &mut Vec<Equation>,id: usize, regex: Regex<String>){
        for eq in equations.iter_mut(){
            if let Some(var_id) = eq.right_side.iter().position(|f| f.var == Some(id)){
                eq.right_side[var_id].regex = Regex::concatenate_vec(vec![eq.right_side[var_id].regex.clone(),regex.clone()]);
                eq.right_side[var_id].var = None;
            }

            eq.factorize();
        }

    }



    pub fn substitute_in_other_equations(equations: &mut Vec<Equation>,eqn_to_substitute: &Equation){
        for eq in equations.iter_mut(){
            if let Some(var_id) = eq.right_side.iter().position(|f| f.var == Some(eqn_to_substitute.left_side)){
                let reg_old_coefficient = eq.right_side[var_id].regex.clone();
                eq.right_side.remove(var_id);

                for new_factor in eqn_to_substitute.right_side.iter(){
                    eq.right_side.push(Factor{regex:Regex::concatenate_vec(vec![reg_old_coefficient.clone(), new_factor.regex.clone()]),var: new_factor.var.clone()});
                }
            }

            eq.factorize();
        }
    }
    fn build_equation_system_from_automaton(aut: &Automaton, r_matrix: &Vec<Vec<Regex<String>>>,nodes: &Vec<NodeIndex>) -> Vec<Equation>{
        let mut equations: Vec<Equation> = vec![];

        for (idx,line) in r_matrix.iter().enumerate(){

            let left_side = idx;
            let mut right_side = Vec::new();

            for (other_idx,reg) in line.iter().enumerate(){
                if reg != &Regex::Empty{
                    let factor = Factor{regex:reg.clone(),var:Some(other_idx)};
                    right_side.push(factor);
                }
            }
            //We add Epsilons to the equations of final states
            if aut.final_states.contains(&nodes[idx]){
                right_side.push(Factor{regex:Regex::Epsilon,var:None});
            }

            //If final states are not precised (transition systems)
            if right_side.len() == 0{
                let factor = Factor{regex:Regex::Epsilon,var:None};
                right_side.push(factor);
            }

            equations.push(Equation::new(left_side,right_side));
        }

        equations
    }

    fn print_equation(equations: &Vec<Equation>){
        for eq in equations{
            println!("{}",eq);
        }
    }
}



impl fmt::Display for Factor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let buff =if let Some(v) = self.var{
            format!("{}R{}",self.regex.to_text(Some('.')),v)
        }
        else{
            format!("{}",self.regex.to_text(Some('.')))
        };

        write!(f,"{}",buff )

    }
}
impl fmt::Display for Equation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let right_side = self.right_side.iter().map(|f| f.to_string()).collect::<Vec<String>>().join(" + ");
        let buff = format!("R{} = {}",self.left_side,right_side);

        write!(f,"{}",buff )

    }
}