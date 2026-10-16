use std::collections::{BTreeSet, HashMap, VecDeque};
use petgraph::stable_graph::{NodeIndex, StableGraph};
use petgraph::visit::EdgeRef;
use crate::automata::automaton::automaton::Automaton;

impl Automaton{
    /// Converts an NFA into a DFA using the Powerset Construction algorithm.
    /// Traverses transitions labeled with standard input symbols as well as epsilon ("").
    pub fn nfa_to_dfa(nfa: &Automaton) -> Automaton {
        // Collect all alphabet symbols (excluding epsilon transitions represented by "")
        let mut alphabet = BTreeSet::new();
        for edge in nfa.graph.edge_weights() {
            if !edge.is_empty() {
                alphabet.insert(edge.clone());
            }
        }

        let mut dfa_graph = StableGraph::<String, String>::new();

        // Map a subset of NFA states (BTreeSet<NodeIndex>) to a single DFA NodeIndex
        let mut dfa_state_map: HashMap<BTreeSet<NodeIndex>, NodeIndex> = HashMap::new();
        let mut worklist: VecDeque<BTreeSet<NodeIndex>> = VecDeque::new();

        // 1. Compute the epsilon-closure of initial states
        let initial_set = Self::epsilon_closure(&nfa.graph, nfa.initial_states.clone());

        // 2. Add the initial state set to the DFA
        let dfa_initial_node = dfa_graph.add_node(format!("{:?}", initial_set));
        dfa_state_map.insert(initial_set.clone(), dfa_initial_node);
        worklist.push_back(initial_set);

        let mut dfa_final_states = Vec::new();
        let nfa_final_set: BTreeSet<NodeIndex> = nfa.final_states.iter().cloned().collect();

        // 3. Main Subset Construction Loop
        while let Some(current_subset) = worklist.pop_front() {
            let current_dfa_node = dfa_state_map[&current_subset];

            // Mark DFA state as final if it contains any NFA final state
            if current_subset.iter().any(|state| nfa_final_set.contains(state)) {
                if !dfa_final_states.contains(&current_dfa_node) {
                    dfa_final_states.push(current_dfa_node);
                }
            }

            // Compute transitions for each symbol in the alphabet
            for symbol in &alphabet {
                let moved = Self::move_set(&nfa.graph, &current_subset, symbol);
                let next_subset = Self::epsilon_closure(&nfa.graph, moved.clone());

                if next_subset.is_empty() {
                    continue;
                }

                // Get existing DFA node or create a new one
                let next_dfa_node = *dfa_state_map.entry(next_subset.clone()).or_insert_with(|| {
                    let new_node = dfa_graph.add_node(format!("{:?}", next_subset));
                    worklist.push_back(next_subset);
                    new_node
                });

                // Add transition edge to DFA
                dfa_graph.add_edge(current_dfa_node, next_dfa_node, symbol.clone());
            }
        }

        Automaton {
            initial_states: vec![dfa_initial_node],
            final_states: dfa_final_states,
            graph: dfa_graph,
        }
    }

    /// Computes the epsilon-closure of a given set of NFA states.
    fn epsilon_closure<S>(graph: &StableGraph<String, String>, states: S) -> BTreeSet<NodeIndex>
    where
        S: IntoIterator<Item = NodeIndex>,
    {
        let mut closure: BTreeSet<NodeIndex> = states.into_iter().collect();
        let mut stack: Vec<NodeIndex> = closure.iter().cloned().collect();

        while let Some(node) = stack.pop() {
            for edge in graph.edges(node) {
                // Epsilon transitions are represented by an empty string ""
                if edge.weight().is_empty() {
                    let target = edge.target();
                    if closure.insert(target) {
                        stack.push(target);
                    }
                }
            }
        }

        closure
    }

    /// Moves from a set of states along edges matching a specific symbol.
    fn move_set(
        graph: &StableGraph<String, String>,
        states: &BTreeSet<NodeIndex>,
        symbol: &str,
    ) -> BTreeSet<NodeIndex> {
        let mut target_set = BTreeSet::new();

        for &state in states {
            for edge in graph.edges(state) {
                if edge.weight() == symbol {
                    target_set.insert(edge.target());
                }
            }
        }

        target_set
    }
}