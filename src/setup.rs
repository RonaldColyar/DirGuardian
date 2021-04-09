
use rand::seq::SliceRandom;
use std::fs::File;
use std::io::prelude::*;
pub struct SetupObj{
    pub ip : String,
    pub fake_content : String,
    pub bait_file_names : [String ; 5]
}


impl SetupObj {
    pub fn new()-> Self{
        
        Self{
            ip : "None".to_string(),
            fake_content : "YR83ybfi4832jbfs923!!2".to_string() , 
            bait_file_names : [
                "DirPassword.txt".to_string() , 
                "DirPassCode.txt".to_string() , 
                "DirAccessCode.txt".to_string() ,
                "DirPass.txt".to_string() , 
                "CodeToGainAccess.txt".to_string()]
        }
    }
    pub fn create_new_bait_file(&mut self) -> std::io::Result<bool> {
        if (true){
            //random file name
            let file_name = self.bait_file_names.choose(&mut rand::thread_rng()).unwrap();
            File::create(file_name)?.write_all(self.fake_content.as_bytes())?;

            Ok(true)
         

        }
        else{
            Ok(true)

        }
    }

    }




