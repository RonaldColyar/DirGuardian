
use std::fs::File;
use std::io::Read;


pub struct Router{
    successful_attempts :i8,
    failed_attempts : i8
}

impl Router{
    pub fn new(s:i8 , f:i8) -> Self{
        Self{
            successful_attempts : s,
            failed_attempts : f
        }
    }

    fn check_param_count(&mut self ,status : &mut bool , command :&Vec<&str>){
            if command.len() > 4 ||  command.len() <3 {
                *status = false;

            }
    }
    fn check_file(&mut self, status : &mut bool , command :Vec<&str>){
                    // if there was already an error
                    if *status != false{
                        let path = command[1];
                        let result = || ->std::result::Result<() , std::io::Error>{
                            let mut f = File::open(path)?;
                            Ok(())
                        };
                        //if there is an error
                        if let Err(_err) =result(){
                            *status = false;
                        };
                        
                    }
                
                    
                }

    
    fn command_is_valid(&mut self , command : Vec<&str>)-> bool{
        let mut status = true;
        self.check_param_count(&mut status , &command);
        self.check_file(&mut status , command);
        return status;
    }
    pub fn route_command(&mut self, input:String){
       let  command_vec =  input.as_str().split(" ").collect();
       if self.command_is_valid(command_vec) == true {
        
       }
       else{
           
       }

    }


}
