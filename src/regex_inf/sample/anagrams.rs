use std::collections::HashSet;
use itertools::Itertools;
use crate::regex_inf::sample::sample::Sample;

impl Sample {

    pub fn get_2grams(&self)->Vec<(String,String)>{// TO TEST
        let mut return_set = Vec::new();

        for string in self.sample.iter(){
            if string.is_empty(){
                continue;
            }
            if self.separator == None {
                /*If there are no separator, we collect the 2-grams of characters */
                let mut separated_iterator = string.chars();
                let size = separated_iterator.clone().count();

                //Some duplicated code here, to optimize
                for j in 0..size{
                    if let (Some(a),Some(b)) = (separated_iterator.clone().nth(j),separated_iterator.clone().nth(j+1)) {

                        return_set.push((a.to_string(),b.to_string()));
                    }
                }
            }
            else {
                /*If there is a separator, we collect the 2-grams after dislocation with respect to the separator*/
                let mut separated_iterator = string.split_terminator(self.separator.unwrap());

                let size = separated_iterator.clone().count();

                //Some duplicated code here, to optimize
                for j in 0..size{
                    if let (Some(a),Some(b)) = (separated_iterator.clone().nth(j),separated_iterator.clone().nth(j+1)) {
                        let mut temp = String::from(a);

                        return_set.push((a.to_string(),b.to_string()));
                    }
                }

            }
        }

        return_set
    }

    /*
    pub fn get_2grams_old(&self)->HashSet<String>{// TO TEST
        let mut return_set: HashSet<String> = HashSet::new();

        for string in &self.sample{
            if string.is_empty(){
                continue;
            }
            if self.separator == None {
                /*If there are no separator, we collect the 2-grams of characters */
                let mut separated_iterator = string.chars();
                let size = separated_iterator.clone().count();

                //Some duplicated code here, to optimize
                for j in 0..size{
                    if let (Some(a),Some(b)) = (separated_iterator.clone().nth(j),separated_iterator.clone().nth(j+1)) {

                        let mut temp = String::from(a);
                        temp.push(b);
                        return_set.insert(temp);
                    }
                }
            }
            else {
                /*If there is a separator, we collect the 2-grams after dislocation with respect to the separator*/
                let mut separated_iterator = string.split_terminator(self.separator.unwrap());

                let size = separated_iterator.clone().count();

                //Some duplicated code here, to optimize
                for j in 0..size{
                    if let (Some(a),Some(b)) = (separated_iterator.clone().nth(j),separated_iterator.clone().nth(j+1)) {
                        let mut temp = String::from(a);
                        temp.push(self.separator.unwrap());
                        temp.push_str(b);
                        return_set.insert(temp);
                    }
                }

            }
        }

        return_set
    }

     */

    pub fn get_kgrams(&self,k: usize)->HashSet<String>{
        let mut return_set: HashSet<String> = HashSet::new();

        for string in &self.sample{
            if string.is_empty(){
                continue;
            }
            if self.separator == None {
                /*If there are no separator, we collect the k-grams of characters */
                let mut separated_iterator = string.chars().collect_vec();
                let size = separated_iterator.clone().len();

                //Some duplicated code here, to optimize
                for j in 0..size{
                    if size-j >=k {
                        let mut temp_k_gram = String::from("");
                        for l in j..j+k{
                            temp_k_gram.push(separated_iterator[l]);
                        }
                        return_set.insert(temp_k_gram);
                    }
                }
            }
            else {
                /*If there is a separator, we collect the k-grams after dislocation with respect to the separator*/
                let mut separated_iterator = string.split_terminator(self.separator.unwrap()).collect_vec();

                let size = separated_iterator.clone().len();

                //Some duplicated code here, to optimize
                for j in 0..size{
                    if size-j >=k {
                        let mut temp_k_gram = String::from("");
                        for l in j..j+k{
                            temp_k_gram.push_str(separated_iterator[l]);
                            if l<j+k-1{
                                temp_k_gram.push(self.separator.unwrap());
                            }
                        }
                        return_set.insert(temp_k_gram);
                    }
                }

            }
        }

        return_set
    }

}