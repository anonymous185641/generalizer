use std::collections::HashMap;
use crate::terms::substitution::substitution::Substitution;
use crate::terms::substitution::variable::Variable;
use crate::terms::function::{Axioms, Function, FunctionSignature};
use crate::terms::term::Term;


/// Entry point: determines whether a pattern `p` matches a subject `s`
/// modulo associative and/or commutative properties.
pub fn brute_force_match_modulo_ac(p: &Term, s: &Term) -> bool {
    let mut subst = Substitution::new();
    match_term(p, s, &mut subst)
}

pub fn bf_match_ac(p: &Term, s: &Term) -> Option<Substitution> {
    //println!("Here Match AC:");
    let mut subst = Substitution::new();
   
    if match_term(p, s, &mut subst){
        //println!("OUT");
        Some(subst)
    }
    else{
        //println!("OUT");
        None
    }
}

/// Recursive matching function
fn match_term(p: &Term, s: &Term, subst: &mut Substitution) -> bool {
    match (p, s) {
        // -------------------
        // Case 1: variable pattern
        // -------------------
        (Term::Variable(v), _) => match subst.map.get(v) {
            Some(existing) => existing == s,
            None => {
                subst.map.insert(v.clone(), s.clone());
                true
            }
        },

        // -------------------
        // Case 2: function vs function
        // -------------------
        (Term::Function(f1), Term::Function(f2)) => {
            if f1.signature.name != f2.signature.name {
                return false;
            }

            let axioms = &f1.signature.axioms;

            // --- Associative + Commutative ---
            if axioms.contains(&Axioms::A) && axioms.contains(&Axioms::C) {
                let args1 = flatten_ac(f1);
                let args2 = flatten_ac(f2);
                let save = subst.clone();
                if match_ac_args(&args1, &args2, &f1.signature, subst) {
                    return true;
                } else {
                    *subst = save;
                    return false;
                }
            }

            // --- Associative only ---
            if axioms.contains(&Axioms::A) && !axioms.contains(&Axioms::C) {
                let args1 = flatten_ac(f1);
                let args2 = flatten_ac(f2);
                let save = subst.clone();
                if match_a_args(&args1, &args2, &f1.signature, subst) {
                    return true;
                } else {
                    *subst = save;
                    return false;
                }
            }

            // --- Commutative only (binary) ---
            if axioms.contains(&Axioms::C) {
                if f1.args.len() != 2 || f2.args.len() != 2 {
                    return false;
                }
                return (match_term(&f1.args[0], &f2.args[0], subst)
                    && match_term(&f1.args[1], &f2.args[1], subst))
                    || (match_term(&f1.args[0], &f2.args[1], subst)
                    && match_term(&f1.args[1], &f2.args[0], subst));
            }

            // --- No axioms (free function symbol) ---
            if f1.args.len() != f2.args.len() {
                return false;
            }
            for (a, b) in f1.args.iter().zip(f2.args.iter()) {
                if !match_term(a, b, subst) {
                    return false;
                }
            }
            true
        }

        // Otherwise mismatch
        _ => false,
    }
}

/// Flatten nested associative/AC nodes (f(f(a,b),c) → f(a,b,c))
fn flatten_ac(f: &Function) -> Vec<Term> {
    let mut result = Vec::new();
    for arg in &f.args {
        if let Term::Function(inner) = arg {
            if inner.signature.name == f.signature.name
                && inner.signature.axioms.contains(&Axioms::A)
            {
                result.extend(flatten_ac(inner));
                continue;
            }
        }
        result.push(arg.clone());
    }
    result
}

/// Generate canonical string for sorting arguments under commutativity.
fn canonical_key(term: &Term) -> String {
    match term {
        Term::Variable(v) => format!("{}_{}", v.label, v.id),
        Term::Function(f) => {
            let args = f
                .args
                .iter()
                .map(canonical_key)
                .collect::<Vec<_>>()
                .join(",");
            format!("{}({})", f.signature.name, args)
        }
    }
}

/// Combine a list of terms into one AC term (e.g. [a,b,c] → f(a,b,c))
fn build_ac_term(signature: &FunctionSignature, parts: &[Term]) -> Term {
    if parts.len() == 1 {
        parts[0].clone()
    } else {
        Term::Function(Function {
            signature: signature.clone(),
            args: parts.to_vec(),
        })
    }
}

//
// =========================
//   AC MATCHING (A + C)
// =========================
//
fn match_ac_args(
    p_args: &[Term],
    s_args: &[Term],
    signature: &FunctionSignature,
    subst: &mut Substitution,
) -> bool {
    // Split pattern args into variables and non-variables
    let mut nonvar_indices = Vec::new();
    let mut vars = Vec::new();
    let mut var_index = HashMap::new();

    for (i, p) in p_args.iter().enumerate() {
        if let Term::Variable(v) = p {
            if !var_index.contains_key(v) {
                var_index.insert(v.clone(), vars.len());
                vars.push(v.clone());
            }
        } else {
            nonvar_indices.push(i);
        }
    }

    // No variables => compare as multisets
    if vars.is_empty() {
        if p_args.len() != s_args.len() {
            return false;
        }
        let mut pa = p_args.to_vec();
        let mut sa = s_args.to_vec();
        pa.sort_by_key(|t| canonical_key(t));
        sa.sort_by_key(|t| canonical_key(t));
        let save = subst.clone();
        for (pp, ss) in pa.iter().zip(sa.iter()) {
            if !match_term(pp, ss, subst) {
                *subst = save;
                return false;
            }
        }
        return true;
    }

    // General AC case: variables can absorb multiple unordered subject elements
    let n_subjects = s_args.len();
    if nonvar_indices.len() > n_subjects {
        return false;
    }

    let mut used = vec![false; n_subjects];
    try_assign_nonvars(
        0,
        &nonvar_indices,
        p_args,
        s_args,
        &mut used,
        signature,
        &vars,
        subst,
        true, // unordered (AC)
    )
}

//
// =========================
//   A MATCHING (Associative only)
// =========================
//
fn match_a_args(
    p_args: &[Term],
    s_args: &[Term],
    signature: &FunctionSignature,
    subst: &mut Substitution,
) -> bool {
    // Purely associative, preserve argument order
    match_a_seq(p_args, s_args, 0, 0, signature, subst)
}

/// Sequential associative matching (ordered, variables absorb contiguous segments)
fn match_a_seq(
    p_args: &[Term],
    s_args: &[Term],
    i: usize,
    j: usize,
    signature: &FunctionSignature,
    subst: &mut Substitution,
) -> bool {
    if i == p_args.len() && j == s_args.len() {
        return true;
    }
    if i == p_args.len() || j == s_args.len() {
        return false;
    }

    match &p_args[i] {
        Term::Variable(v) => {
            // Try all non-empty contiguous slices of subject args
            for end in (j + 1)..=s_args.len() {
                let slice = &s_args[j..end];
                let term = build_ac_term(signature, slice);
                let save = subst.clone();

                if let Some(existing) = subst.map.get(v) {
                    if existing != &term {
                        continue;
                    }
                } else {
                    subst.map.insert(v.clone(), term);
                }

                if match_a_seq(p_args, s_args, i + 1, end, signature, subst) {
                    return true;
                }
                *subst = save;
            }
            false
        }
        Term::Function(_) => {
            if match_term(&p_args[i], &s_args[j], subst) {
                match_a_seq(p_args, s_args, i + 1, j + 1, signature, subst)
            } else {
                false
            }
        }
    }
}

//
// =========================
//  Shared helper (used by AC)
// =========================
//
fn try_assign_nonvars(
    ni: usize,
    nonvar_indices: &Vec<usize>,
    p_args: &[Term],
    s_args: &[Term],
    used: &mut Vec<bool>,
    signature: &FunctionSignature,
    vars: &Vec<Variable>,
    subst: &mut Substitution,
    unordered: bool,
) -> bool {
    if ni == nonvar_indices.len() {
        // All non-vars assigned → distribute leftovers to variables
        let mut leftovers = Vec::new();
        for (i, u) in used.iter().enumerate() {
            if !*u {
                leftovers.push(i);
            }
        }
        if leftovers.len() < vars.len() {
            return false;
        }
        let mut buckets: Vec<Vec<Term>> = vec![Vec::new(); vars.len()];
        return assign_leftovers_recursive(
            0,
            &leftovers,
            s_args,
            vars,
            &mut buckets,
            signature,
            subst,
            unordered,
        );
    }

    let idx = nonvar_indices[ni];
    let p_term = &p_args[idx];
    let save_subst = subst.clone();

    for s_idx in 0..s_args.len() {
        if used[s_idx] {
            continue;
        }
        if match_term(p_term, &s_args[s_idx], subst) {
            used[s_idx] = true;
            if try_assign_nonvars(
                ni + 1,
                nonvar_indices,
                p_args,
                s_args,
                used,
                signature,
                vars,
                subst,
                unordered,
            ) {
                return true;
            }
            used[s_idx] = false;
        }
        *subst = save_subst.clone();
    }
    false
}

fn assign_leftovers_recursive(
    pos: usize,
    leftovers: &Vec<usize>,
    s_args: &[Term],
    vars: &Vec<Variable>,
    buckets: &mut Vec<Vec<Term>>,
    signature: &FunctionSignature,
    subst: &mut Substitution,
    unordered: bool,
) -> bool {
    if pos == leftovers.len() {
        // All assigned → ensure non-empty buckets and check consistency
        if buckets.iter().any(|b| b.is_empty()) {
            return false;
        }

        // If unordered, sort each bucket canonically for consistency
        if unordered {
            for b in buckets.iter_mut() {
                b.sort_by_key(|t| canonical_key(t));
            }
        }

        let save = subst.clone();
        for (vi, var) in vars.iter().enumerate() {
            let term = build_ac_term(signature, &buckets[vi]);
            if let Some(existing) = subst.map.get(var) {
                if existing != &term {
                    *subst = save;
                    return false;
                }
            } else {
                subst.map.insert(var.clone(), term);
            }
        }
        return true;
    }

    let idx = leftovers[pos];
    for var_i in 0..vars.len() {
        buckets[var_i].push(s_args[idx].clone());
        if assign_leftovers_recursive(
            pos + 1,
            leftovers,
            s_args,
            vars,
            buckets,
            signature,
            subst,
            unordered,
        ) {
            return true;
        }
        buckets[var_i].pop();
    }
    false
}































































/*
/// Entry point: determines whether a pattern `p` matches a subject `s`
/// modulo associative and/or commutative properties.
pub fn match_modulo_ac(p: &Term, s: &Term) -> bool {
    let mut subst = Substitution {
        map: HashMap::new(),
    };
    match_term(p, s, &mut subst)
}

/// Recursive helper implementing matching modulo AC.
/// Returns true if there exists a substitution σ such that pσ = s.
fn match_term(p: &Term, s: &Term, subst: &mut Substitution) -> bool {
    match (p, s) {
        // ----------------------------
        // Case 1: Pattern is a variable
        // ----------------------------
        (Term::Variable(v), _) => match subst.map.get(v) {
            // Variable already mapped → must match consistently
            Some(existing) => existing == s,
            // Unbound variable → record new binding
            None => {
                subst.map.insert(v.clone(), s.clone());
                true
            }
        },

        // ----------------------------
        // Case 2: Pattern and subject are functions
        // ----------------------------
        (Term::Function(f1), Term::Function(f2)) => {
            // Different function symbols cannot match
            if f1.signature.name != f2.signature.name {
                return false;
            }

            let axioms = &f1.signature.axioms;

            // -------- AC (Associative + Commutative) case --------
            if axioms.contains(&Axioms::A) && axioms.contains(&Axioms::C) {
                let mut args1 = flatten_ac(f1);
                let mut args2 = flatten_ac(f2);

                if args1.len() != args2.len() {
                    return false;
                }

                // Sort both argument lists to compare in canonical order
                args1.sort_by_key(|t| canonical_key(t));
                args2.sort_by_key(|t| canonical_key(t));

                for (a, b) in args1.iter().zip(args2.iter()) {
                    if !match_term(a, b, subst) {
                        return false;
                    }
                }
                true
            }

            // -------- C (Commutative only) case --------
            else if axioms.contains(&Axioms::C) {
                if f1.args.len() != 2 || f2.args.len() != 2 {
                    return false;
                }

                // Try both argument orders
                (match_term(&f1.args[0], &f2.args[0], subst)
                    && match_term(&f1.args[1], &f2.args[1], subst))
                    || (match_term(&f1.args[0], &f2.args[1], subst)
                    && match_term(&f1.args[1], &f2.args[0], subst))
            }

            // -------- A (Associative only) case --------
            else if axioms.contains(&Axioms::A) {
                let args1 = flatten_ac(f1);
                let args2 = flatten_ac(f2);
                if args1.len() != args2.len() {
                    return false;
                }
                for (a, b) in args1.iter().zip(args2.iter()) {
                    if !match_term(a, b, subst) {
                        return false;
                    }
                }
                true
            }

            // -------- No Axioms (Free function) --------
            else {
                if f1.args.len() != f2.args.len() {
                    return false;
                }
                for (a, b) in f1.args.iter().zip(f2.args.iter()) {
                    if !match_term(a, b, subst) {
                        return false;
                    }
                }
                true
            }
        }

        // ----------------------------
        // Case 3: Different kinds (variable vs function)
        // ----------------------------
        _ => false,
    }
}

/// Flatten nested arguments under an associative or AC function.
/// Example: +(a, +(b, c)) → [a, b, c]
fn flatten_ac(f: &Function) -> Vec<Term> {
    let mut result = Vec::new();
    for arg in &f.args {
        if let Term::Function(inner) = arg {
            if inner.signature.name == f.signature.name
                && inner.signature.axioms.contains(&Axioms::A)
            {
                result.extend(flatten_ac(inner));
                continue;
            }
        }
        result.push(arg.clone());
    }
    result
}

/// Generate a canonical string key for ordering arguments deterministically.
/// Used for sorting commutative arguments.
fn canonical_key(term: &Term) -> String {
    match term {
        Term::Variable(v) => format!("{}_{}", v.label, v.id),
        Term::Function(f) => {
            let args = f
                .args
                .iter()
                .map(canonical_key)
                .collect::<Vec<_>>()
                .join(",");
            format!("{}({})", f.signature.name, args)
        }
    }
}

 */