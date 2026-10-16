use std::collections::{HashMap, HashSet,VecDeque};
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::prelude::StableGraph;
use crate::regex_inf::regex::regex::Regex;

#[derive(Debug,Clone)]
pub struct KOA{
    pub nb_occurrence: usize,
    pub koa: DiGraph<Regex<String>,()>,
}


impl KOA{
    pub fn new()-> KOA{
        KOA{
            nb_occurrence: 1,
            koa: DiGraph::new(),
        }
    }
}
/*
impl KOA {
    pub fn new()-> KOA{
        KOA{
            nb_occurrence: 1,
            koa: StableGraph::new(),
            alphabet: Vec::new(),
            code_matrix: Vec::new(),
            lab: HashMap::new(),
            code: Vec::new(),
        }
    }
    pub fn gen_koa(sample: &Sample, k: usize)->KOA{
        todo!();
    }

    pub fn init(k: usize,alphabet: &Vec<String>)->Vec<Vec<usize>>{//OK
        //Here we initialize 10 individuals
        let mut P : Vec<Vec<usize>> = Vec::new();
        let size_matrix = alphabet.len()*k + 1;
        let mut rng = rand::thread_rng();
        
        for i in 0..10{
            let mut individual: Vec<usize> = Vec::new();

            for j in 0..size_matrix*size_matrix{
                individual.push(rng.gen_range(0..2));
            }
            P.push(individual);
        }
        P
    }

    pub fn decode(k:usize, code: &Vec<usize>,alphabet: &Vec<String>)->KOA{ //OK
        //This function transform a code vector into a KOA

        let size_matrix = alphabet.len()*k + 1;

        if code.is_empty(){
            return Self::new();
        }

        //We reconstruct the adjacency matrices
        let mut code_matrix :Vec<Vec<usize>>= Vec::new();

        for i in 0..size_matrix{
            let mut temp = Vec::new();
            for j in (i*size_matrix)..(i+1)*size_matrix{
                temp.push(code[j]);
            }
            code_matrix.push(temp);
        }

        //Handling the graph
        let mut koa = StableGraph::new();

         //Nodes
        let mut lab:HashMap<String,Vec<NodeIndex>> = HashMap::new();
            //SRC
        let src = koa.add_node(Regex::SRC);
        //lab.insert(src,Regex::SRC);
            //OTHER NODES
        for letter in alphabet{
            let mut temp_nodes_indices = Vec::new();
            for i in 0..k{
                temp_nodes_indices.push(koa.add_node(Regex::Literal(letter.clone())));
            }
            lab.insert(letter.clone(),temp_nodes_indices);

        }
            //SNK
        let snk = koa.add_node(Regex::SNK);
        //lab.insert(snk,Regex::SNK);


         //Edges
        for i in 0..size_matrix{
            for j in 0..size_matrix{
                if code_matrix[i][j]==1{
                    if (i == 0)&& (j<size_matrix-1) { //src to string node
                       // let n_1 = nodes_indices_map.get(&alphabet[((i+2)/k).floor()]).unwrap()[(i+2)%k];
                        let node = lab.get(&alphabet[(j/k)as usize]).unwrap()[j%k]; //FLOOR

                        koa.add_edge(src,node,());
                    }
                    else if (i==0)&&(j==size_matrix-1){ //src to snk
                        koa.update_edge(src,snk,());
                    }
                    else if (i > 0)&& (i<size_matrix-1)&& (j<size_matrix-1){// String node to String node
                        let node_1 = lab.get(&alphabet[((i-1)/k) as usize]).unwrap()[(i-1)%k]; //FLOOR
                        let node_2 = lab.get(&alphabet[(j/k) as usize]).unwrap()[j%k]; //FLOOR
                        koa.add_edge(node_1,node_2,());
                    }
                    else if (i > 0)&& (i<size_matrix-1)&& (j==size_matrix-1) { //String node to snk
                        let node = lab.get(&alphabet[((i-1)/k) as usize]).unwrap()[(i-1)%k]; //FLOOR
                        koa.add_edge(node,snk,());
                    }
                }
            }
        }

        KOA{
            nb_occurrence: k,
            koa: koa,
            alphabet: alphabet.clone(),
            code_matrix: code_matrix,
            lab: lab,
            code: code.clone(),
        }

    }
    /*
    pub fn encode(k:usize,koa: &StableGraph<Regex<String>,()>,alphabet: &Vec<String>)-> Vec<usize>{
        unimplemented!();
    }
    pub fn code_to_matrix(k:usize,code: &Vec<usize>,alphabet: &Vec<String>)->Vec<Vec<usize>>{
        unimplemented!();
    }

     */
    /*
    pub fn run_stable_graph(graph: &mut StableGraph<Regex<String>,()>, word: &String, separator: Option<char>)->Option<Vec<NodeIndex>>{
        let src = Automaton::get_node_by_weight(graph,&Regex::SRC).unwrap();
        let snk = Automaton::get_node_by_weight(graph,&Regex::SNK).unwrap();

        let mut word_splitted_char = Vec::new();
        let mut word_splitted_str = Vec::new();
        if separator==None{
            word_splitted_char = word.chars().collect_vec();
        }
        else{
             word_splitted_str = word.split_terminator(separator.unwrap()).collect_vec();
        }

        let mut path :Vec<NodeIndex> = Vec::new();

        let mut dfs = Dfs::new(&graph.clone(),src);
        let mut ct:usize = 0;
        while let Some(node) = dfs.next(&*graph) {
            if separator == None{
                if *graph.node_weight(node).unwrap()==Regex::Literal(word_splitted_char[ct].to_string()){
                    path.push(node.clone());
                    ct+=1;
                }
                else if node == snk {
                    break;
                }
            }
            else{
                if *graph.node_weight(node).unwrap()==Regex::Literal(word_splitted_str[ct].to_string()){
                    path.push(node.clone());
                    ct+=1;
                }
                else if node == snk {
                    break;
                }
            }

        }


        if path.len()==word_splitted_str.len() || path.len()==word_splitted_char.len(){
            return Some(path);
        }
        None
    }
    pub fn run_di_graph(graph: &mut DiGraph<Regex<String>,()>, word: &String, separator: Option<char>)->Option<Vec<NodeIndex>>{
        let src = SOA::get_node_index_from_weight(&graph,Regex::SRC).unwrap();
        let snk = SOA::get_node_index_from_weight(&graph,Regex::SNK).unwrap();

        let mut word_splitted_char = Vec::new();
        let mut word_splitted_str = Vec::new();

        if separator==None{
            word_splitted_char = word.chars().collect_vec();
            let mut path :Vec<NodeIndex> = Vec::new();

            /////////////////////////////////
            let mut current_node = src.clone();
            for letter in word_splitted_char{
                for edge in graph.edges_directed(current_node,Direction::Outgoing){
                    if let Some((_,target)) = graph.edge_endpoints(edge.id()){
                        if graph.node_weight(target).unwrap()==word_splitted_char[ct]{
                            current_node = target;
                        }
                    }
                }
            }
        }
        else{
            word_splitted_str = word.split_terminator(separator.unwrap()).collect_vec();
        }


        None
    }
     */

    pub fn calcFitness(koa: &KOA, sample: &Sample)->f32{
        let mut fitness : f32 = 0.0;
        let mut true_positive = HashSet::new();
        let mut false_negative = HashSet::new();

        for word in &sample.sample{
            if Self::run_stable_graph(&mut koa.koa.clone(),word,sample.separator)!=None{
                true_positive.insert(word.clone());
            }
            else {
                false_negative.insert(word.clone());
            }
        }



        fitness = (true_positive.len() as f32 - false_negative.len() as f32)/(sample.sample.len() as f32);

        fitness
    }



    //pub fn select(P: &Vec<Vec<usize>>,values: &Vec<f32>)->(usize,usize){   //Return a pair of parents
    pub fn select(values: &Vec<f32>)->(usize,usize){   //Return a pair of parents
        let mut rgn = rand::thread_rng();

        let mut koas: Vec<KOA> = Vec::new();

        let max_value_index = Self::max_index(values).unwrap();
        let mut other_value = rgn.gen_range(0..values.len());

        while other_value == max_value_index{
            other_value = rgn.gen_range(0..values.len());
        }
        (max_value_index,other_value)
    }

    pub fn crossover(P: &Vec<Vec<usize>>,parent_1_id: usize,parent_2_id: usize)->Vec<Vec<usize>>{
        let mut rgn = rand::thread_rng();

        let mut new_population = P.clone();
        let mut parent_1 = P[parent_1_id].clone();
        let mut parent_2 = P[parent_2_id].clone();

        let start = rgn.gen_range(0..parent_1.len()/2);
        let terminal = rgn.gen_range(start+1..parent_1.len());

        //We permute the values between start and terminal
        for i in start..=terminal{
            let temp = parent_1[i];
            parent_1[i] = parent_2[i];
            parent_2[i] = temp;
        }

        new_population[parent_1_id] = parent_1;
        new_population[parent_2_id] = parent_2;
        new_population
    }
    pub fn mutate(individual: &Vec<usize>)->Vec<usize>{
        let mut new_individual = individual.clone();

        let mut rgn = rand::thread_rng();
        let random_index = rgn.gen_range(0..individual.len());

        new_individual[random_index] = (new_individual[random_index]+1)%2;

        new_individual
    }

    pub fn bestFA(koas: &Vec<KOA>,sample: &Sample)->KOA{
        if koas.is_empty(){
            return Self::new();
        }
        let mut values = Vec::new();

        for koa in koas {
            values.push(Self::calcFitness(koa,sample));
        }
        koas[Self::max_index(&values).unwrap()].clone()
    }
    fn max_index(numbers: &Vec<f32>) -> Option<usize> {
        if numbers.is_empty() {
            return None;
        }
        let mut max_index = 0;
        for (i, &value) in numbers.iter().enumerate() {
            if value > numbers[max_index] {
                max_index = i;
            }
        }
        Some(max_index)
    }
    pub fn disambiguate(koa: &KOA, sample: &Sample)-> KOA{
        let mut Q = VecDeque::new();
        let mut B:HashSet<NodeIndex> = HashSet::new();
        let mut new_koa = koa.clone();

        let src = Automaton::get_node_by_weight(&koa.koa,&Regex::SRC).unwrap();
        Q.push_front(src);

        while !Q.is_empty(){
            let s = Q.front().unwrap().clone();


            while !Self::out(&new_koa.koa,&s.clone(),&sample.alphabet.iter().cloned().collect_vec().clone()).is_empty(){
                let mut C: Vec<KOA> = Vec::new();

                //We select a random
                let out_map_s = Self::out(&new_koa.koa,&s.clone(),&sample.alphabet.iter().cloned().collect_vec().clone());
                let out_set = out_map_s.get(out_map_s.clone().keys().collect_vec()[0]).unwrap().clone();

                //Loop
                for t in out_set.clone(){
                    let mut new_koa_prime = new_koa.clone();
                    for t_prime in Self::exclude_element_in_vector(&out_set.clone(),&t.clone()){
                        new_koa_prime.delete_edge_in_koa(&s,&t_prime);
                    }
                    C.push(new_koa_prime);
                }
                new_koa = Self::bestFA(&C,sample);

            }

            let out_map_s = Self::out(&new_koa.koa,&s.clone(),&sample.alphabet.iter().cloned().collect_vec().clone());
            B.push(s); //We push s to B
            Q.pop_front(); // We pop s from Q
            //Then, we enqueue in Q the elements of out(s) that are not in B
            for val_vector in out_map_s.values(){
                for element in val_vector{
                    if !B.contains(element){
                        Q.push_front(element.clone());
                    }
                }
            }

        }

        new_koa
    }


    pub fn simplify(koa: &KOA, sample: &Sample)-> KOA{
        todo!();
    }
    pub fn mark_koa(k_oa: &KOA)-> (SOA,HashMap<String,String>){
        todo!();
    }




    ////////////////////////////////////////////////////////////////
    //MINOR FUNCTIONS
    ///////////////////////////////////////////////////////////////
    pub fn out(graph: &StableGraph<Regex<String>,()>,node: &NodeIndex, alphabet: &Vec<String>)->HashMap<Regex<String>,Vec<NodeIndex>>{
        let mut output = HashMap::new();

        //Initialization
        for letter in alphabet{
            output.insert(Regex::Literal(letter.clone()),Vec::new());
        }

        for edge in graph.edges_directed(node.clone(),Direction::Outgoing){
            if let Some((_,target)) = graph.edge_endpoints(edge.id()){
                let target_label = graph.node_weight(target).unwrap().clone();
                if target_label!= Regex::SNK{
                    output.get_mut(&target_label).unwrap().push(target);
                }
            }
        }

        output
    }
    pub fn exclude_element_in_vector<T: PartialEq+Clone>(vector: &Vec<T>, element: &T)->Vec<T>{
        let mut output = Vec::new();

        for value in vector{
            if *value!=*element{
                output.push(value.clone());
            }
        }
        output
    }
    pub fn delete_edge_in_koa(&mut self,source: &NodeIndex,target: &NodeIndex)->KOA{
        let mut new_graph = self.koa.clone();

        if let Some(e) = new_graph.find_edge(source.clone(),target.clone()){
            new_graph.remove_edge(e);
        }

        KOA{
            nb_occurrence: self.clone().nb_occurrence,
            koa: new_graph.clone(),
            alphabet: self.clone().alphabet,
            code_matrix: self.code_matrix.clone(),//WRONG BUT DON'T MATTER
            lab: self.lab.clone(),
            code: self.code.clone(),//WRONG BUT DON'T MATTER
        }
    }

    /////////////////////////////////////////////////////////////
    ////UTILITIES FOR RUN
    ////////////////////////////////////////////////////////////
    pub fn first(graph: &StableGraph<Regex<String>,()>)->HashMap<Regex<String>,Vec<NodeIndex>>{
        unimplemented!();
    }

    pub fn last(graph: &StableGraph<Regex<String>,()>)->HashMap<Regex<String>,Vec<NodeIndex>>{
        unimplemented!();
    }

    pub fn follow(graph: &StableGraph<Regex<String>,()>,node: &NodeIndex)-> HashMap<Regex<String>,Vec<NodeIndex>>{
        unimplemented!();
    }

}

 */

