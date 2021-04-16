
use std::fs::File;
use std::io::Read;
use crate::setup::SetupObj;
use crate::logger::Logger;
use crate::sockethandler::SockHandler;
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
    fn check_file(&mut self, status : &mut bool , command :&Vec<&str>){
                    if *status != false{
                        let path = command[1];
                        let result = || ->std::result::Result<() , std::io::Error>{
                            let mut f = File::open(path)?;
                            Ok(())
                        };
                        if let Err(_err) =result(){
                            self.logger.file_not_found(command[1]);
                            *status = false;
                        };   
                    }    
                }



    fn command_is_valid(&mut self , command : &Vec<&str>)-> bool{
        let mut status = true;
        self.check_param_count(&mut status , &command);
        //only check file path if leading command is encrypt
        if status == true {
            if command[0] == "encrypt"{
                self.check_file(&mut status , command);
            }
        }
        return status;
    }
    
    fn new_input(&mut self,prompt : &str) -> String{
        print!("{}", prompt);
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input);
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

    fn encrypt_offline(&mut self ,data:Vec<&str>){


    }

    fn route_command_tier_one(&mut self , command_vec:Vec<&str>){

        if command_vec[0] == "encrypt" && command_vec[2] == "off"{
            //route to offline encryption
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
