use std::hash::Hash;
use std::convert::From;
use itertools::Itertools;
use rand::prelude::IndexedRandom;


#[derive(Debug)]
pub struct Sample{
    pub alphabet: Vec<String>,
    pub sample: Vec<String>,
    pub separator: Option<char>,//Separation symbol between alphabet letters
}



impl Sample{
    pub fn new()->Sample{ //OK
        //Create a new empty sample
        Sample{
            alphabet: Vec::new(),
            sample: Vec::new(),
            separator : None,
        }
    }
    pub fn new_from_set(string_set: &Vec<String>, separator: Option<char>)-> Sample{ //TO TEST
        //Create a Sample from vector of strings
        let mut return_sample = Self::new();

        if !string_set.is_empty(){

            for string in string_set{
                if string.is_empty(){
                    continue;
                }
                if separator == None {
                    /*If there are no separator, we dislocate the string into chars
                     and add each char as alphabet symbols*/
                     let mut char_iterator = string.chars();
                      while let Some(c) = char_iterator.next() {
                          return_sample.alphabet.push(c.to_string());
                      }
                }
                else {
                    /*If there is a separator, we split the strings according the separator
                     and then add the obtained strings as alphabet symbols*/
                    let mut separated = string.split_terminator(separator.unwrap());

                    while let Some(s) = separated.next(){
                        return_sample.alphabet.push(s.to_string());
                    }

                }

             }

            return_sample.sample = string_set.clone();
            return_sample.separator = separator;

            return_sample.alphabet.sort();
            return_sample.alphabet.dedup();

            return_sample.sample.sort();
            return_sample.sample.dedup();
        }



        return_sample
    }

    pub fn first_symbols(&self)->Vec<String>{ //TO TEST
        //Returns the first alphabet symbols of strings of the sample
        let mut return_set = Vec::new();

            for string in &self.sample{
                if string.is_empty(){
                    return_set.push("".to_string());
                }
                if self.separator == None {
                    /*If there are no separator, we add the first characters of strings
                    as first symbols */
                    let mut char_iterator = string.chars();

                    if let Some(c) = char_iterator.nth(0){
                        return_set.push(c.to_string());
                    }


                }
                else {
                    /*If there is a separator, we just add the first character after splitting
                    as first symbols*/
                    let mut separated = string.split_terminator(self.separator.unwrap());
                    if let Some(s) = separated.nth(0){
                        return_set.push(s.to_string());
                    }

                }
            }

        return_set
    }
    pub fn last_symbols(&self)->Vec<String>{ //TO TEST
        //Returns the last alphabet symbols of strings of the sample
        let mut return_set = Vec::new();

        for string in &self.sample{
            if string.is_empty(){
                return_set.push("".to_string());;
            }
            if self.separator == None {
                /*If there are no separator, we add the last characters of strings
                as last symbols */
                let mut char_iterator = string.chars();

                if let Some(c) = char_iterator.last(){
                    return_set.push(c.to_string());
                }


            }
            else {
                /*If there is a separator, we just add the last characters after splitting
                as last symbols*/
                let mut separated = string.split_terminator(self.separator.unwrap());
                if let Some(s) = separated.last(){
                    return_set.push(s.to_string());
                }

            }
        }

        return_set
    }

    /*
    pub fn filter_old(&self, symbols_to_retain: HashSet<String>) -> Sample{ //To test
        /*  This function extract a substring from a string in a way shown in the following example from the paper of Zhang et al(2018) the paper ,

	        if P = {a,d,f} and s = {rrabbfffcq}, we have s.filter(P) = {afff}
	    */
        let mut new_sample:Sample = Self::new();

        for string in &self.sample{
            let mut new_string = String::from("");

            if self.separator==None{
                new_string = string.clone();
                new_string.retain(|c| symbols_to_retain.contains(&c.to_string()));
            }
            else {
                let mut separated_iterator = string.split_terminator(self.separator.unwrap());
                let mut filtered_iterator = separated_iterator.filter(|s| symbols_to_retain.contains(*s));

                while let Some(s) = filtered_iterator.next(){
                    new_string.push_str(s);
                    new_string.push(self.separator.unwrap());
                }
                if new_string.ends_with(self.separator.unwrap()){
                    new_string.pop();
                }
            }
            new_sample.sample.push(new_string);

        }

        new_sample.separator = self.separator;
        new_sample.alphabet = symbols_to_retain;
        new_sample
    }

     */

    pub fn filter(&self, symbols_to_retain: Vec<String>) -> Sample{ //To test
        /*  This function extract a substring from a string in a way shown in the following example from the paper of Zhang et al(2018) the paper ,

	        if P = {a,d,f} and s = {rrabbfffcq}, we have s.filter(P) = {afff}
	    */
        let mut new_sample:Sample = Self::new();

        for string in &self.sample{
            let mut new_string = String::from("");

            if self.separator==None{
                new_string = string.clone();
                new_string.retain(|c| symbols_to_retain.contains(&c.to_string()));
            }
            else {
                let mut separated_iterator = string.split_terminator(self.separator.unwrap());
                let mut filtered_iterator = separated_iterator.filter(|s| symbols_to_retain.contains(&s.to_string()));

                while let Some(s) = filtered_iterator.next(){
                    new_string.push_str(s);
                    new_string.push(self.separator.unwrap());
                }
                if new_string.ends_with(self.separator.unwrap()){
                    new_string.pop();
                }
            }
            //
            /*Error
            if !new_string.is_empty(){
                new_sample.sample.push(new_string);
            }
             */
            new_sample.sample.push(new_string);

        }

        new_sample.separator = self.separator;
        new_sample.alphabet = Vec::from_iter(symbols_to_retain);

        new_sample.sample.sort();
        new_sample.sample.dedup();


        new_sample
    }



}