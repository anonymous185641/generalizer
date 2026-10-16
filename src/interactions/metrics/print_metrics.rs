use std::fs::File;
use std::io::Write;
use crate::interactions::syntax::interaction::Interaction;
use crate::regex_inf::sample::sample::Sample;

impl Interaction{
    pub fn write_metrics_all(&self,parent_dir: &str,file_name: &str)->std::io::Result<()>{
        let metrics = format!("Semantic size: {},\n\
                                     Size: {}",
                                    self.semantic_size(),self.size());

        let path_str = format!("{}/{}.txt",parent_dir,file_name);
        let mut file = File::create(&path_str)?;
        file.write_all(&metrics.as_bytes())
    }

    pub fn write_metrics_global(&self,global_int: &Interaction, parent_dir: &str,file_name: &str)->std::io::Result<()>{
        let metrics = format!("Semantic size: {},\n\
                                     Distance from global: {},\n\
                                     Size: {}",
                              self.semantic_size(),Interaction::edit_distance(global_int,self),self.size());

        let path_str = format!("{}/{}.txt",parent_dir,file_name);
        let mut file = File::create(&path_str)?;
        file.write_all(&metrics.as_bytes())
    }

}