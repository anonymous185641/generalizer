use crate::anti_unification::generalizer::generalizer::Generalizer;
use crate::matching::brute_force::matching_ac::brute_force_match_modulo_ac;
/* WITH BRUTE FORCE MATCHING */
/**/
pub fn minimise_ac(generalisers: Vec<Generalizer>) -> Vec<Generalizer> {
    let mut r: Vec<Generalizer> = Vec::new();

    // -----------------------------
    // Step 1: Remove duplicates
    // -----------------------------
    for s in &generalisers {
        let mut found = false;

        for r_item in &r {
            // If the terms are mutually AC-matchable, we consider them duplicates

            if brute_force_match_modulo_ac(&r_item.t, &s.t) && brute_force_match_modulo_ac(&s.t, &r_item.t) {
                found = true;
                break;
            }
        }

        if !found {
            r.push(s.clone());
        }
    }

    // -----------------------------
    // Step 2: Remove subsumed elements
    // -----------------------------

    /*
    for el in &r{
        println!("{}", el.t);
    }
     */
    let mut s_min: Vec<Generalizer> = Vec::new();

    for s in &r {

        let mut redundant = false;

        for r_item in &r {
            // Skip self-comparison
            //println!("Before:: {} <? {} : {}",r_item.t,s.t,match_modulo_ac(&r_item.t, &s.t));

            if std::ptr::eq(s, r_item) {
                continue;
            }

            // If r_item.t AC-matches s.t, then s is redundant (subsumed)

            //println!("After:: {} <? {} : {}",r_item.t,s.t,match_modulo_ac(&r_item.t, &s.t));
            /*
            if match_modulo_ac(&r_item.t, &s.t) {
                redundant = true;
                break;
            }
             */
            if brute_force_match_modulo_ac(&s.t, &r_item.t) {
                redundant = true;
                break;
            }
        }

        if !redundant {
            s_min.push(s.clone());
        }
    }

    s_min
}



/* WITH CONTEJEAN MATCHING

pub fn minimise_ac(generalisers: Vec<Generaliser>) -> Vec<Generaliser> {
    let mut r: Vec<Generaliser> = Vec::new();
    /*
    for el in &generalisers {
        println!("{}", el.t);
    }
     */
    // -----------------------------
    // Step 1: Remove duplicates
    // -----------------------------
    for s in &generalisers {
        let mut found = false;

        for r_item in &r {
            // If the terms are mutually AC-matchable, we consider them duplicates

            if MatchingProcess::contejean_algorithm(&r_item.t, &s.t) && MatchingProcess::contejean_algorithm(&s.t, &r_item.t) {
                found = true;
                break;
            }
        }

        if !found {
            r.push(s.clone());
        }
    }

    // -----------------------------
    // Step 2: Remove subsumed elements
    // -----------------------------

    /*
    for el in &r{
        println!("{}", el.t);
    }

     */

    let mut s_min: Vec<Generaliser> = Vec::new();

    for s in &r {

        let mut redundant = false;

        for r_item in &r {
            // Skip self-comparison
            println!("Before:: {} <? {} : {}",r_item.t,s.t,MatchingProcess::contejean_algorithm(&r_item.t, &s.t));

            if std::ptr::eq(s, r_item) {
                continue;
            }

            // If r_item.t AC-matches s.t, then s is redundant (subsumed)

            //println!("After:: {} <? {} : {}",r_item.t,s.t,match_modulo_ac(&r_item.t, &s.t));
            /*
            if match_modulo_ac(&r_item.t, &s.t) {
                redundant = true;
                break;
            }
             */
            if MatchingProcess::contejean_algorithm(&s.t, &r_item.t) {
                redundant = true;
                break;
            }
        }

        if !redundant {
            s_min.push(s.clone());
        }
    }

    s_min
}

 */