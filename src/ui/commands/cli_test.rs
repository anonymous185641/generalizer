use std::fs;
use std::io::Write;
use clap::ArgMatches;
use crate::automata::automaton::automaton::Automaton;
use crate::automata::cfsm::system_cfsm::system_cfsm::SystemCFSM;
use crate::automata::cfsm::system_cfsm_io::parsing_system_cfsm::cfsm_interface::cfsm_parse_file;
use crate::automata::communicating_automata::communicating_automata_io::parsing_system_caut::comm_aut_interface::communicating_automata_parse_file;
use crate::automata::communicating_automata::product::sync_product::sync_product::SyncProduct;
use crate::automata::communicating_automata::system_cautomata::cautomata::SystemCommunicatingAutomata;
use crate::matching::matching_ac::matching_process::match_ac::match_ac;
use crate::terms::function::{Function, FunctionSignature};
use crate::terms::substitution::variable::Variable;
use crate::terms::term::Term;

pub fn cli_test(_matches: &ArgMatches) {

    /*
    let sample = vec!["begk".to_string(),
                                         "aabengk".to_string(),
                                         "abegjj".to_string(),
                                          "beg".to_string(),
                                          "hk".to_string(),
                                          "behgj".to_string(),
                                           "belhg".to_string(),
                                            "bheg".to_string(),
                                            "bfcmd".to_string(),
                                            "bfdm".to_string(),
                                             "afmcd".to_string(),
                                             "adf".to_string()];



    //let sample  = vec!["l1!m1.l1?m3".to_string(),"l1?m2.l1?m3".to_string()];

    //let sample1  = vec!["ab".to_string(),"cdddba".to_string()];

    let sampl = Sample::new_from_set(&sample, None);

    //let sampl1 = Sample::new_from_set(&sample1, None);

    //println!("tran reduc {:?}",sampl.tran_reduction());

    let con_miner = ConMiner::con_miner(&sampl).simplify();
    let crx_chare = CRX::crx(&sampl).simplify();
    let chare = Soa2Chare::i_chare(&sampl).simplify();
    let ichare = GenIchare::i_ichare(&sampl).simplify();
    let sore = Soa2Sore::i_sore(&sampl).simplify();
    let soire = Soa2Soire::i_soire(&sampl).simplify();

    println!("con_miner: {}",con_miner.to_text(None));
    println!("Language size: {}", con_miner.language_size(&sampl.alphabet));
    println!("Data cost: {}", con_miner.data_cost(&sampl));
    println!("Combinatorial cardinality: {}", con_miner.combinatorial_cardinality());
    println!("--------------------------------------");

    println!("crx_chare: {}",crx_chare.to_text(None));
    println!("Language size: {}", crx_chare.language_size(&sampl.alphabet));
    println!("Data cost: {}", crx_chare.data_cost(&sampl));
    println!("Combinatorial cardinality: {}", crx_chare.combinatorial_cardinality());

    println!("--------------------------------------");
    println!("chare: {}",chare.to_text(None));
    println!("Language size: {}", chare.language_size(&sampl.alphabet));
    println!("Data cost: {}", chare.data_cost(&sampl));
    println!("Combinatorial cardinality: {}", chare.combinatorial_cardinality());

    println!("--------------------------------------");
    println!("ichare: {}",ichare.to_text(None));
    println!("Language size: {}", ichare.language_size(&sampl.alphabet));
    println!("Data cost: {}", ichare.data_cost(&sampl));
    println!("Combinatorial cardinality: {}", ichare.combinatorial_cardinality());

    println!("--------------------------------------");
    println!("sore: {}",sore.to_text(None));
    println!("Language size: {}", sore.language_size(&sampl.alphabet));
    println!("Data cost: {}", sore.data_cost(&sampl));
    println!("Combinatorial cardinality: {}", sore.combinatorial_cardinality());

    println!("--------------------------------------");
    println!("soire: {}",soire.to_text(None));
    println!("Language size: {}", soire.language_size(&sampl.alphabet));
    println!("Data cost: {}", soire.data_cost(&sampl));
    println!("Combinatorial cardinality: {}", soire.combinatorial_cardinality());


    /*
    let mut graph:UnGraph<String,()> = UnGraph::new_undirected();

    let a = graph.add_node("a".to_string());
    let b = graph.add_node("b".to_string());
    //let c = graph.add_node("c".to_string());
    //let d = graph.add_node("d".to_string());

    graph.add_edge(a,b,());
    //graph.add_edge(c,d,());

    let mis = Sample::clique_removal(&graph);

    println!("mis: {:?}",mis);


    let tt = Sample::undirected_subgraph(&graph,&b,false);
    println!("{:?}", Dot::with_config(&tt, &[Config::EdgeNoLabel]));


    //let mut complement = Sample::complement_undirected(&graph);

    //petgraph::operator::complement(&graph,&mut complement,());

    //println!("{:?}", Dot::with_config(&complement, &[Config::EdgeNoLabel]));
    */

    /*
     let sample = vec!["ab".to_string(),"bc".to_string(),"ac".to_string()];

    let sampl = Sample::new_from_set(&sample,None);

    let soa = SOA::build_from_sample(&sampl);

    println!("{:?}", Dot::with_config(&soa.soa, &[Config::EdgeNoLabel]));
    println!("Old");
    println!("{:?}",soa.build_level_order_old());

    println!("New");
    println!("{:?}",soa.build_level_order());

     */

     */






    /*
    let sample = Sample::new_from_set(&vec!["abb".to_string(),
                                            "aaaab".to_string(),
                                            "aabbb".to_string()],None);

    let r1 = Regex::Kleene(Box::new(Regex::Or(vec![Regex::Literal("a".to_string()),Regex::Literal("b".to_string())])));
    let r2 = Regex::Concat(vec![Regex::Kleene(Box::new(Regex::Literal("a".to_string()))),Regex::Kleene(Box::new(Regex::Literal("b".to_string())))]);
    let r3 = Regex::Concat(vec![Regex::Plus(Box::new(Regex::Literal("a".to_string()))),Regex::Plus(Box::new(Regex::Literal("b".to_string())))]);


    println!("r1 ------ Size= {} ------- Datacost= {} --------- CC= {}",r1.language_size(2),r1.data_cost(&sample),r1.combinatorial_cardinality());
    println!("r2 ------ Size= {} ------- Datacost= {} --------- CC= {}",r2.language_size(2),r2.data_cost(&sample),r2.combinatorial_cardinality());
    println!("r3 ------ Size= {} ------- Datacost= {} --------- CC= {}",r3.language_size(2),r3.data_cost(&sample),r3.combinatorial_cardinality());

    */
    /*
    let r1 = Regex::Kleene(Box::new(Regex::Or(vec![Regex::Literal("a".to_string()),Regex::Literal("b".to_string())])));
    let r2 = Regex::Int(vec![Regex::Kleene(Box::new(Regex::Literal("a".to_string()))),Regex::Kleene(Box::new(Regex::Literal("b".to_string())))]);
    let r3 = Regex::Or(vec![Regex::Int(vec![Regex::Kleene(Box::new(Regex::Literal("a".to_string()))),Regex::Kleene(Box::new(Regex::Literal("b".to_string())))]),
                             Regex::Kleene(Box::new(Regex::Literal("b".to_string())))]);

    println!("Lenght r1: {} ---- sym= {} ----- meta={}",r1.len(),r1.sym(),r1.meta());
    println!("Lenght r2: {}",r2.len());
    println!("Lenght r3: {}",r3.len());

     */
    /*
    let sample = Sample::new_from_set(&vec!["aaa".to_string(),
                                            "aabac".to_string(),
                                            "aaaabaaaac".to_string(),
                                            "bcaaa".to_string()],None);

    let ka = Regex::Kleene(Box::new(Regex::Literal("a".to_string())));
    let qm_bc = Regex::Qm(Box::new(Regex::Concat(vec![Regex::Literal("b".to_string()),Regex::Literal("c".to_string())])));

    let r1 = Regex::Kleene(Box::new(Regex::Or(vec![Regex::Literal("a".to_string()),Regex::Literal("b".to_string()),Regex::Literal("c".to_string())])));

    let r2 = Regex::Concat(vec![Regex::Int(vec![ka.clone(),qm_bc.clone()]), ka.clone()]) ;

    let r3 = Regex::Int(vec![ka.clone(),qm_bc.clone()]);

    println!("r1 ------ Size= {} ------- Datacost= {} --------- CC= {}",r1.language_size(2),r1.data_cost(&sample),r1.combinatorial_cardinality());
    println!("r2 ------ Size= {} ------- Datacost= {} --------- CC= {}",r2.language_size(2),r2.data_cost(&sample),r2.combinatorial_cardinality());
    println!("r3 ------ Size= {} ------- Datacost= {} --------- CC= {}",r3.language_size(2),r3.data_cost(&sample),r3.combinatorial_cardinality());

     */

    /*
    let a1 = Action::new(1,1,ActionType::Emission);

    let i1 = Interaction::wrap_loop(&Interaction::Action(a1));

    println!("{:?}",i1.semantic_size());

     */

/*




    let sample = vec!["l2?close.l2!busy.l2!msg".to_string(),
                      "l2?close.l2!busy.l2!msg.l2?close.l2!busy.l2!msg".to_string(),
                      "l2?cwin.l2!blose.l2!busy.l2!msg".to_string(),
                      "l2?cwin.l2!blose.l2!busy.l2!msg.l2?close.l2!busy.l2!msg".to_string(),
                      "l2!busy.l2?close.l2!msg".to_string(),
                        "l2!busy.l2?close.l2!msg.l2?close.l2!busy.l2!msg".to_string(),
                        "l2!busy.l2?close.l2!msg".to_string(),
                        "l2!busy.l2?close.l2!msg.l2?close.l2!busy.l2!msg".to_string(),];




    let sampl = Sample::new_from_set(&sample, Some('.'));

    //let sampl1 = Sample::new_from_set(&sample1, None);

    //println!("tran reduc {:?}",sampl.tran_reduction());

    println!(" {:?} ",sampl.sample);
    let con_miner = ConMiner::con_miner(&sampl);
    let crx_chare = CRX::crx(&sampl);
    let chare = Soa2Chare::i_chare(&sampl);
    let ichare = GenIchare::i_ichare(&sampl);
    let sore = Soa2Sore::i_sore(&sampl);
    let soire = Soa2Soire::i_soire(&sampl);
    let state_remov_reg = KTssiStateRemoval::k_tssi_state_removal(2,&sampl);
    let brzozowski_reg = KTssiBrzozowski::k_tssi_brzozowski(2,&sampl);
    let transitive_closure_reg = KTssiTransitiveClosure::k_tssi_transitive_closure(2,&sampl);

    println!("con_miner: {}",con_miner.to_text(Some('.')));
    println!("--------------------------------------");

    println!("crx_chare: {}",crx_chare.to_text(Some('.')));

    println!("--------------------------------------");
    println!("Soa2Chare: {}",chare.to_text(Some('.')));

    println!("--------------------------------------");
    println!("GenIchare: {}",ichare.to_text(Some('.')));

    println!("--------------------------------------");
    println!("Soa2Sore: {}",sore.to_text(Some('.')));

    println!("--------------------------------------");
    println!("Soa2Soire: {}",soire.to_text(Some('.')));

    println!("--------------------------------------");
    println!("KTssiStateRemoval: {}",state_remov_reg.to_text(Some('.')));
    println!("--------------------------------------");

    println!("KTssiTransitiveClosure: {}",transitive_closure_reg.to_text(Some('.')));
    println!("--------------------------------------");

    println!("KTssiBrzozowski: {}",brzozowski_reg.to_text(Some('.')));
    println!("--------------------------------------");

    let automata = KTSSI::ktssi(2,&sampl);

    let output_dir = "Test_dir";
    let _ = fs::remove_dir(output_dir);
    let _ = fs::create_dir(output_dir);

    let mut graph_file = fs::File::create(format!("{}/automaton.dot",output_dir)).unwrap();
    graph_file.write_all(automata.to_dot().to_dot_string().as_bytes()).unwrap();
    graph_file.flush().unwrap();

    let mut gen_ctx = GeneralContext::new();
    gen_ctx.add_lf("l2".to_string());
    gen_ctx.add_msg("close".to_string());
    gen_ctx.add_msg("busy".to_string());
    gen_ctx.add_msg("msg".to_string());
    gen_ctx.add_msg("cwin".to_string());
    gen_ctx.add_msg("blose".to_string());


    let int_state_removal = format!("{}/int_state_removal",output_dir);
    let _ = fs::create_dir(&int_state_removal);
    draw_model(&gen_ctx,&"int_sr".to_string(),&int_state_removal,&Interaction::regex_to_interaction_non_formatted(&gen_ctx,&state_remov_reg).unwrap());

    let int_trans_closure = format!("{}/int_trans_closure",output_dir);
    let _ = fs::create_dir(&int_trans_closure);
    draw_model(&gen_ctx,&"int_tc".to_string(),&int_trans_closure,&Interaction::regex_to_interaction_non_formatted(&gen_ctx,&transitive_closure_reg).unwrap());

    let int_bro = format!("{}/int_bro",output_dir);
    let _ = fs::create_dir(&int_bro);
    draw_model(&gen_ctx,&"int_bro".to_string(),&int_bro,&Interaction::regex_to_interaction_non_formatted(&gen_ctx,&brzozowski_reg).unwrap());

*/

    /* */

    /*
    let dot_content = r#"
digraph {
    rankdir=LR;

    init [shape=point];
    init -> q0;

    q0 [shape=circle];
    q1 [shape=circle];
    q2 [shape=circle];
    q3 [shape=doublecircle];

    q0 -> q1 [label="a"];
    q1 -> q2 [label="b"];
    q2 -> q3 [label="c"];
    q3 -> q3 [label="c"];
}
"#;
     */

    /*
    let automaton = Automaton::from_dot_file("test_aut.dot").unwrap();



    /*
    let mut graph = petgraph::stable_graph::StableGraph::<String,String>::new();

    let q0 = graph.add_node("q0".to_string());
    let q1 = graph.add_node("q1".to_string());

    graph.add_edge(q0,q0,"a".to_string());
    graph.add_edge(q0,q1,"b".to_string());
    graph.add_edge(q1,q1,"b".to_string());

    let automaton = Automaton::new_from_fields(graph,vec![q0],vec![q1],AutomatonType::NDEF);
    */
    println!("{:?}",automaton);
    println!("");
    println!("Reg1 Transitive closure: {:?}",automaton.transitive_closure().simplify());
    println!("-------------------------------");
    println!("Reg2 State removal: {:?}",Automaton::dfa_to_regex_state_removal(&automaton));
    println!("-------------------------------");
    //println!("Reg3: {:?}",Automaton::dfa_to_regex_brzozowski(&automaton));
    //println!("Reg3 Bro: {:?}",automaton.to_regex_brzozowski());

    println!();
    println!("Test new Brzozowski");
    automaton.to_regex_brzozowski();
    println!()

     */


    //Test automaton parsing
    /**/
    let a = communicating_automata_parse_file("bargain.caut");

   let sync = SyncProduct::build_sync_product(&a.unwrap()).unwrap();
    
    sync.draw(&"TestSync".to_string(),&"temp1".to_string());
    

    /*
    //Test Variant parsing
    let x1 = Variable::new("x".to_string(), 1);
    let x2 = Variable::new("x".to_string(), 2);
    let x1_term = Term::Variable(x1.clone());
    let x2_term = Term::Variable(x2.clone());
    let seq = FunctionSignature::new("alt".to_string(),2,vec![]);
    let term = Term::Function(Function::new(&seq,&vec![x1_term, x2_term]));
    let empty = FunctionSignature::new("Empty".to_string(),0,vec![]);
    println!("{}",term.to_string());
    let sig = Term::special_get_interaction_signature(&vec![term.clone()]);
    let variants = Variant::get_variants(&term,&sig,&vec![x1, x2]);

    for v in variants {
        println!("{}",v);
        println!("\n----------------\n");
    }
*/


    /*
    //Test matching
    let x1 = Variable::new("x".to_string(), 1);
    let x2 = Variable::new("x".to_string(), 2);
    let x1_term = Term::Variable(x1.clone());
    let x2_term = Term::Variable(x2.clone());
    let f = FunctionSignature::new("f".to_string(),2,vec![]);
    let g = FunctionSignature::new("g".to_string(),2,vec![]);
    let f_x1_x2 = Term::Function(Function::new(&f,&vec![x1_term, x2_term]));

    let a_sig = FunctionSignature::new("a".to_string(),0,vec![]);
    let a_term = Term::Function(Function::new(&a_sig,&vec![]));

    let b_sig = FunctionSignature::new("b".to_string(),0,vec![]);
    let b_term = Term::Function(Function::new(&b_sig,&vec![]));

    let f_a_b = Term::Function(Function::new(&f,&vec![a_term.clone(), b_term.clone()]));
    let g_a_b = Term::Function(Function::new(&g,&vec![a_term, b_term]));

    let subs = match_ac(&f_x1_x2,&g_a_b,false);
    println!("{}",subs[0]);

     */

    /*
    //Test cfsm to caut
    let cfsm = cfsm_parse_file("Bargain").unwrap();

    let caut = SystemCommunicatingAutomata::cfsm_to_caut(&cfsm);

    caut.to_caut_file("OUTPUT_BARGAIN_TRANSLATION",None);

     */


}