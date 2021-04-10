
use std::fs::File;
use std::io::Read;

use crate::logger::Logger;
use crate::sockethandler::SockHandler;

pub struct Router{
    successful_attempts :i8,
    failed_attempts : i8,
    pub logger : Logger,
    socket_handler : SockHandler
}

impl Router{
    pub fn new(s:i8 , f:i8) -> Self{
        Self{
            successful_attempts : s,
            failed_attempts : f,
            logger : Logger::new(0),
            socket_handler : SockHandler::new()
        }
        
    }

    fn check_param_count(&mut self ,status : &mut bool , command :&Vec<&str>){
            if   command.len() != 3  {
                *status = false;

            }
    }
    fn check_file(&mut self, status : &mut bool , command :Vec<&str>){
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

    fn init_socket_connection(&mut self) -> bool{
        let result = self.socket_handler.connect();

        if result.is_ok(){
            return true;
        }
        else{
            return false;
        }
    }
    fn command_is_valid(&mut self , command : Vec<&str>)-> bool{
        let mut status = true;
        self.check_param_count(&mut status , &command);
        self.check_file(&mut status , command);
        return status;
    }
    fn route_command_tier_one(&mut self , command_vec:Vec<&str>){


        if command_vec[0] == "encrypt" && command_vec[2] == "off"{
            //route to offline encryption
        }
        else if command_vec[0] == "encrypt" && command_vec[2] == "on"{
            //route to online encryption
        }
    }
    pub fn route_command(&mut self, input:String){
       let  command_vec =  input.as_str().split(" ").collect();
       if self.command_is_valid(command_vec) == true {

       }
       else{
           
       }

    }


}
