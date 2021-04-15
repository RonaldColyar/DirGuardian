
use rand::seq::SliceRandom;
use std::fs::File;
use std::io::prelude::*;
use std::option;
use std::env;
use crate::jsonhandler::JsonHandler;

pub struct SetupObj{
    pub ip : String,
    pub fake_content : String,
    pub bait_file_names : [String ; 5],
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
                "CodeToGainAccess.txt".to_string()],
          
        }
    }

    pub fn create_new_config(&mut self , data : String) -> std::io::Result<bool>{
        File::create("config_dir.txt")?.write_all(data.as_bytes())?;
        Ok(true)
    }
     fn try_to_access_config_data(&mut self)->
            std::result::Result<std::fs::File , std::io::Error>{
        let path = 
            env::current_dir()
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned() + "/src/config.json";
            let f = File::open( path);
        Ok(f.unwrap())
    }

  
    pub fn color_preference(&mut self)-> String{
        let file = self.config_file();
  
        if file.is_some(){
            let mut content_string = String::new();
            file.unwrap().read_to_string(&mut content_string); 
            let handler =  JsonHandler::new_json_obj(content_string);
            return "place".to_owned();

        }
        else{
            return "issue".to_owned();
        }
    }
    fn config_file(&mut self) -> Option<std::fs::File>{
       let result =  self.try_to_access_config_data();
       if result.is_ok(){
           return Some(result.unwrap());

       }
       else{
           None
       }
    }


    pub fn create_new_bait_file(&mut self) -> std::io::Result<bool> {
            //random file name
            let file_name = self.bait_file_names.choose(&mut rand::thread_rng()).unwrap();
            File::create(file_name)?.write_all(self.fake_content.as_bytes())?;

            Ok(true)
         
    }

    }




