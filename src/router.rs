
use std::fs::File;
use std::io::Read;
use crate::setup::SetupObj;
use crate::logger::Logger;
use crate::sockethandler::SockHandler;
use crate::encryptor;
use crate::sockethandler;
use std::io::Write;
use json;



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
    // used for checking command parameters
    fn check_param_count(&mut self ,status : &mut bool , command :&Vec<&str>){
            if   command.len() < 3   {
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
        println!("  ");
        return input.trim().to_string();
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
    //gather/place encryption key from/on server and decrypt/encrypt the directory
    fn crypt_online(&mut self, data:Vec<&str> ,command:&str  ,fun : fn(&str,&str) ){
        let mut handler = sockethandler::SockHandler::new();
        let mut data_holder = json::JsonValue::new_object();
        data_holder["password"] = data[3].into();
        data_holder["command"] = command.into();
        let stringified_json =  data_holder.dump();
        let response = handler.send_request_and_gather_response(stringified_json);
        if response == "issue"{
            self.logger.failed_request();
        }
        else{
            fun(data[1] , data[3]);
        }
    }

    /* 
    1.This method takes in a path to a directory
    that is going to be encrypted or decrypted[path]

    2. Takes in a function  , which is either encryption or decryption[fun]

    3.Takes in an optional key which is when the user manually
    types a key into the command line.[optional_key]
    */
    fn validate_key_and_crypt( &mut self ,path:&str , fun : fn(&str , &str),key:&str){
        //if there is a option key passed in use it as the key.
        if key.len()>0
        {
            fun(path,key);
            self.logger.complete_cryption("Directory Completely decrypted!!");
    
        }
        else{
            self.logger.invalid_key_size();
        }
       
    }

    fn prompt_for_typed_encrypt_key_and_continue(&mut self,data:Vec<&str>){
        let type_key_response = self.new_input("Would to provide your own key?[Y/N]>");
        let mut key = String::new();
        if type_key_response == "Y"{
            key = self.new_input("Custom Key>")
        }
        else{
            key = encryptor::encryptor::minimal_encryption_key()
        }
        self.logger.log_encryption_key(key.as_str());
        self.validate_key_and_crypt(data[1] ,
            encryptor::encryptor::encrypt_dir_and_sub_dirs,
            key.as_str()

        )
     
    }
    
    fn decrypt_offline(&mut self, data:Vec<&str>){
 
            let key = self.new_input("Key >");
            self.validate_key_and_crypt(data[1],
            encryptor::encryptor::decrypt_dir_and_sub_dirs,key.as_str());
    
       

    }


    fn route_command_tier_one(&mut self , command_vec:Vec<&str>){
        if command_vec[0] == "encrypt" && command_vec[2] == "off"{
            self.prompt_for_typed_encrypt_key_and_continue(command_vec);
        }
        else if command_vec[0] == "decrypt" && command_vec[2] == "off"{
            self.decrypt_offline(command_vec);
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
