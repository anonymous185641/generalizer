
use petgraph::visit::{EdgeRef, IntoEdgeReferences};
use crate::petri_nets::petri_net::petri_net::{PetriNet, PetriNode};
use std::fmt::{Debug, Write};
use std::hash::Hash;
use itertools::Itertools;
use crate::automata::automaton::automaton::Automaton;
use crate::petri_nets::petri_net::place_transition::PetriNetTransitionLabelBound;
use crate::petri_nets::petri_net_io::petri_net_parsing::petri_nets_string_parser::LiteralFromString;
use crate::regex_inf::regex::regex::Regex;

impl<T: PetriNetTransitionLabelBound> PetriNet<T> {
    pub fn to_dot(&self) -> String {
        let mut dot = String::new();

        writeln!(&mut dot, "digraph PetriNet {{").unwrap();
        writeln!(&mut dot, "    rankdir=LR;").unwrap();
        writeln!(&mut dot).unwrap();

        // Places
        for place in &self.places {
            let tokens = self
                .initial_marking
                .tokens
                .get(place)
                .map(|t| t.0)
                .unwrap_or(0);

            let node_name = format!("p{}", place.id.0);

            if tokens > 0 {
                let mut tokens_str = String::from("");
                for _ in 0..tokens {
                    tokens_str.push_str("●");
                }

                writeln!(
                    &mut dot,
                    r#"    {} [shape=circle, label="{}\n{}"];"#,
                    node_name,
                    escape_dot(&place.label),
                    tokens_str
                )
                    .unwrap();
            } else {
                writeln!(
                    &mut dot,
                    r#"    {} [shape=circle, label="{}"];"#,
                    node_name,
                    escape_dot(&place.label)
                )
                    .unwrap();
            }
        }

        writeln!(&mut dot).unwrap();

        // Transitions
        for transition in &self.transitions {
            let node_name = format!("t{}", transition.id.0);

            writeln!(
                &mut dot,
               // r#"    {} [shape=box, width=0.15, height=0.8, label="", xlabel="{}"];"#,
                r#"    {} [shape=box, width=0.3, height=0.3, label="", xlabel="{}"];"#,
                node_name,
                escape_dot(&transition.label.to_string())
            )
                .unwrap();
        }

        writeln!(&mut dot).unwrap();

        // Edges
        for edge in self.graph.edge_references() {
            let source = &self.graph[edge.source()];
            let target = &self.graph[edge.target()];

            let source_name = match source {
                PetriNode::Place(p) => format!("p{}", p.id.0),
                PetriNode::Transition(t) => format!("t{}", t.id.0),
            };

            let target_name = match target {
                PetriNode::Place(p) => format!("p{}", p.id.0),
                PetriNode::Transition(t) => format!("t{}", t.id.0),
            };

            writeln!(
                &mut dot,
                "    {} -> {};",
                source_name,
                target_name
            )
                .unwrap();
        }

        writeln!(&mut dot, "}}").unwrap();

        dot
    }
}


fn escape_dot(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
}
















