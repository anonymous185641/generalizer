use crate::regex_inf::sample::sample::Sample;

impl Sample{
    pub fn parse_txt_file(file_path: &str) -> Vec<String> {
        let mut sample = vec![];
        let file = std::fs::read_to_string(file_path).expect("Something went wrong reading the input file");
        for line in file.lines() {
            let line_without_spaces = line.replace(" ", "");
            if line_without_spaces.eq("<o>"){
                sample.push("".to_string());
            }
            else{
                sample.push(line.to_string());
            }
        }
        sample
    }
}