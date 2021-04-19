
use std::fs::File;
use std::io::Read;
use crate::setup::SetupObj;
use crate::logger::Logger;
use crate::sockethandler::SockHandler;
use crate::encryptor;
use std::io::Write;





use std::io::stdin;
pub struct Router{
    successful_attempts :i8,
    failed_attempts : i8,
    pub logger : Logger,
    setup_obj : SetupObj,

}

impl Router{
    pub fn new(s:i8 , f:i8) -> Self{
        Self{
            successful_attempts : s,
            failed_attempts : f,
            logger : Logger::new(0),
            setup_obj : SetupObj::new(),

        }
        
    }

    //fn route_color_preference(data:String){}
    fn check_param_count(&mut self ,status : &mut bool , command :&Vec<&str>){
            if   command.len() < 2   {
                *status = false;

            }
    }
    fn check_file(&mut self ,path :&str) -> std::result::Result<File , std::io::Error>{
        let try_file = || ->std::result::Result<File , std::io::Error>{
        let mut f = File::open(path)?;
            Ok(f)
                        };
        let result = try_file();
        if let Err(_err) =&result{
            self.logger.file_not_found(path);
                            
            };   
        return result;
                     
        }



    fn command_is_valid(&mut self , command : &Vec<&str>)-> bool{
        let mut status = true;
        self.check_param_count(&mut status , &command);
        //only check file path if leading command is encrypt
        if status == true {
            if command[0] == "encrypt"{
               let result = self.check_file( command[1]);
               if result.is_err(){
                 status = false;
               }
            }
        }
        return status;
       
    }
    
    fn new_input(&mut self,prompt : &str) -> String{
        print!("{}", prompt);
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input);
        input.trim_right_matches("\r\n");
        return input;
    }


    fn check_type_of_change(&mut self , data :Vec<&str>){
        if data[2] == "color"{
            let result = self.new_input("Color?>");
        }
        else if data[2] == "ip"{
            let result = self.new_input("Ip?>");
        }
        else if data[2] == "displayname"{
            let result = self.new_input("DisplayName>?");
        }
    }

    fn settings_config_routing(&mut self , data: Vec<&str> ){
        if data.len() == 3 {
            self.check_type_of_change(data);
            }

        
        else{
            self.logger.unknown_command();
        }
    }
    fn encrypt_online(&mut self, data:Vec<&str>){

    }

    fn check_key_len_and_continue( &mut self, path :&str , key_holder:String){
             //make sure its not an empty string
             if key_holder.len() > 0 {
                //number of paths 
                encryptor::encryptor::encrypt_dir_and_sub_dirs(path,key_holder.as_str());
            }
            else{
               self.logger.invalid_key_size();
            }
    }
    fn validate_key_and_encrypt( &mut self ,path:&str){
        let result: std::result::Result<File , std::io::Error> = self.check_file(path);
        if result.is_ok(){
            let mut key_holder = String::new();
            result.unwrap().read_to_string( &mut key_holder);
            self.check_key_len_and_continue(path,key_holder);
        }
        else{
            self.logger.invalid_key_path();
        }

       
    }
    //WIP
    fn encrypt_offline(&mut self ,data:Vec<&str>){
        let key_response = self.new_input("Would to provide your own key?[Y/N]>");
        let key_path = self.new_input("Path to key file >");
        if key_response == "Y"{
            self.validate_key_and_encrypt(key_path.as_str());
        }
        else{
            let key = encryptor::encryptor::minimal_encryption_key();
            self.logger.log_encryption_key(key.as_str());
            encryptor::encryptor::encrypt_dir_and_sub_dirs(data[1],key.as_str());
        }



    }

    fn route_command_tier_one(&mut self , command_vec:Vec<&str>){
        println!("{}", command_vec[2] == "off");
        if command_vec[0] == "encrypt" && command_vec[2] == "off"{
            self.encrypt_offline(command_vec);
        }
        else if command_vec[0] == "encrypt" && command_vec[2] == "on"{
            //route to online encryption
        }
        else if command_vec[0] == "config"  && command_vec[1] == "settings"{
            self.settings_config_routing(command_vec);
          
        }
        
    }
    //for tests
    //fn test_config_file(&mut self){
      //  let data = "{color:red}".to_string();
        //let create_result = self.setup_obj.create_new_config(data);
        //if create_result ==  true {
           // println!("setup");
        //}
    //}
    
    pub fn route_command(&mut self, input:&str){
        // split commands into format: [param1,param2,param3]
       let  command_vec =  input.split(" ").collect();
       if self.command_is_valid(&command_vec) == true {
           self.route_command_tier_one(command_vec);
       }
       else{
           self.logger.unknown_command();
       }

    }


}
