use crate::regex_inf::regex::regex::Regex;

impl Regex<String>{
    /*
    pub fn simplify(&self) -> Regex<String> { // To Complete
        //Simplify the current label
        match self {
            Regex::Empty => Regex::Empty,
            Regex::Epsilon => Regex::Epsilon,
            Regex::Literal(a)=> Regex::Literal(a.clone()),
            Regex::Concat(label) => {
                let mut temp_label = Self::remove_epsilon_and_empty(label);
                let mut new_label = Vec::new();

                for el in temp_label{
                    new_label.push(el.clone().simplify());
                }
                Regex::Concat(new_label.clone())
            },
            Regex::Or(label)=> {
                let mut temp_label = Self::remove_empty(label);
                let mut new_label = Vec::new();

                for el in temp_label.iter(){
                    new_label.push(el.clone().simplify());
                }

                Regex::Or(new_label.clone())
                /*
                let mut temp_label = Self::remove_empty(label);
                let mut new_label = Vec::new();

                for el in temp_label.iter(){
                    new_label.push(el.clone().simplify());
                }

                if new_label.contains(&Regex::Epsilon) && Self::contains_kleene(&new_label){
                    let mut qm_labels = Vec::new();

                    for label in new_label.iter(){
                        if *label != Regex::Epsilon && label.is_head_kleene(){
                            qm_labels.push(Regex::Qm(Box::new(label.clone())).simplify());
                        }
                        else if *label != Regex::Epsilon{
                            qm_labels.push(label.clone());
                        }
                    }
                    if qm_labels.len()>1{
                        Regex::Or(qm_labels)
                    }
                    else{
                        qm_labels[0].clone()
                    }
                }
                else if new_label.contains(&Regex::Epsilon) && !Self::contains_kleene(&new_label) && new_label.len() == 2{
                    if new_label[0] == Regex::Epsilon {
                        Self::question_mark(&new_label[1])
                    }
                    else{
                        Self::question_mark(&new_label[0])
                    }
                }
                else{
                    Regex::Or(new_label.clone())
                }
                */
            },
            Regex::Int(label)=>{
                let mut temp_label = Self::remove_epsilon_and_empty(label);
                let mut new_label = Vec::new();

                for el in temp_label{
                    new_label.push(el.clone().simplify());
                }
                Regex::Int(new_label.clone())
            },
            Regex::Kleene(label) =>{
                let mut temp_label = *label.clone();
                Regex::Kleene(Box::new(temp_label.simplify()))
            },
            Regex::Plus(label)=> {
                let mut temp_label = *label.clone();
                match temp_label{
                    Regex::Qm(other_label)=>{
                        let mut other_temp = *other_label.clone();
                        Regex::Kleene(Box::new(other_temp.simplify()))
                    },
                    _ =>Regex::Plus(Box::new(temp_label.simplify())),
                }

            },
            Regex::Qm(label) => {

                let mut temp_label = *label.clone();

                match temp_label{
                    Regex::Plus(other_label)=>{
                        let mut other_temp = *other_label.clone();
                        Regex::Kleene(Box::new(other_temp.simplify()))
                    },
                    _ =>Regex::Qm(Box::new(temp_label.simplify())),
                }
            },
            _ => Regex::Epsilon // for SRC and SNK
        }
    }

     */


    pub fn simplify(&self) -> Regex<String> { // To Complete
        //Simplify the current label
        match self {
            Regex::Empty => Regex::Empty,
            Regex::Epsilon => Regex::Epsilon,
            Regex::Literal(a)=> Regex::Literal(a.clone()),
            Regex::Concat(label) => {
                let mut temp_label = Self::remove_epsilon_and_empty(label);
                //temp_label.sort();
                temp_label.dedup();

                let mut new_label = Vec::new();

                for el in temp_label{
                    new_label.push(el.clone().simplify());
                }
                Regex::Concat(new_label.clone())
            },
            Regex::Or(label)=> {


                let mut temp_label = Self::remove_empty(label);
                temp_label.sort();
                temp_label.dedup();

                let mut new_label = Vec::new();

                for el in temp_label.iter(){
                    new_label.push(el.clone().simplify());
                }

                Regex::Or(new_label.clone())
                /*
                if new_label.contains(&Regex::Epsilon) && Self::contains_kleene(&new_label){
                    let mut qm_labels = Vec::new();

                    for label in new_label.iter(){
                        if *label != Regex::Epsilon && label.is_head_kleene(){
                            qm_labels.push(Regex::Qm(Box::new(label.clone())).simplify());
                        }
                        else if *label != Regex::Epsilon{
                            qm_labels.push(label.clone());
                        }
                    }
                    if qm_labels.len()>1{
                        Regex::Or(qm_labels)
                    }
                    else{
                        qm_labels[0].clone()
                    }
                }
                else if new_label.contains(&Regex::Epsilon) && !Self::contains_kleene(&new_label) && new_label.len() == 2{
                    if new_label[0] == Regex::Epsilon {
                        Self::question_mark(&new_label[1])
                    }
                    else{
                        Self::question_mark(&new_label[0])
                    }
                }
                else{
                    Regex::Or(new_label.clone())
                }

                 */

            },
            Regex::Int(label)=>{
                let mut temp_label = Self::remove_epsilon_and_empty(label);
                temp_label.sort();
                temp_label.dedup();

                let mut new_label = Vec::new();

                for el in temp_label{
                    new_label.push(el.clone().simplify());
                }
                Regex::Int(new_label.clone())
            },
            Regex::Kleene(label) =>{
                let mut temp_label = *label.clone();
                Regex::Kleene(Box::new(temp_label.simplify()))
            },
            Regex::Plus(label)=> {
                let mut temp_label = *label.clone();
                match temp_label{
                    Regex::Qm(other_label)=>{
                        let mut other_temp = *other_label.clone();
                        Regex::Kleene(Box::new(other_temp.simplify()))
                    },
                    _ =>Regex::Plus(Box::new(temp_label.simplify())),
                }

            },
            Regex::Qm(label) => {

                let mut temp_label = *label.clone();

                match temp_label{
                    Regex::Plus(other_label)=>{
                        let mut other_temp = *other_label.clone();
                        Regex::Kleene(Box::new(other_temp.simplify()))
                    },
                    _ =>Regex::Qm(Box::new(temp_label.simplify())),
                }
            },
            _ => Regex::Epsilon // for SRC and SNK
        }
    }

    pub fn simple_simplify(&self) -> Regex<String> {
        //Simplify the current label
        match self {
            Regex::Empty => Regex::Empty,
            Regex::Epsilon => Regex::Epsilon,
            Regex::Literal(a)=> Regex::Literal(a.clone()),
            Regex::Concat(label) => {
                let mut temp_label = Self::remove_epsilon_and_empty(label);
                let mut new_label = Vec::new();

                for el in temp_label{
                    new_label.push(el.clone().simple_simplify());
                }
                Regex::Concat(new_label.clone())
            },
            Regex::Or(label)=> {


                let mut temp_label = Self::remove_empty(label);
                let mut new_label = Vec::new();

                for el in temp_label.iter(){
                    new_label.push(el.clone().simple_simplify());
                }
                Regex::Or(new_label.clone())

            },
            Regex::Int(label)=>{
                let mut temp_label = Self::remove_epsilon_and_empty(label);
                let mut new_label = Vec::new();

                for el in temp_label{
                    new_label.push(el.clone().simple_simplify());
                }
                Regex::Int(new_label.clone())
            },
            Regex::Kleene(label) =>{
                let mut temp_label = *label.clone();
                Regex::Kleene(Box::new(temp_label.simple_simplify()))
            },
            Regex::Plus(label)=> {
                let mut temp_label = *label.clone();
                match temp_label{
                    Regex::Qm(other_label)=>{
                        let mut other_temp = *other_label.clone();
                        Regex::Kleene(Box::new(other_temp.simple_simplify()))
                    },
                    _ =>Regex::Plus(Box::new(temp_label.simple_simplify())),
                }

            },
            Regex::Qm(label) => {

                let mut temp_label = *label.clone();

                match temp_label{
                    Regex::Plus(other_label)=>{
                        let mut other_temp = *other_label.clone();
                        Regex::Kleene(Box::new(other_temp.simple_simplify()))
                    },
                    _ =>Regex::Qm(Box::new(temp_label.simple_simplify())),
                }
            },
            _ => Regex::Epsilon // for SRC and SNK
        }
    }


}