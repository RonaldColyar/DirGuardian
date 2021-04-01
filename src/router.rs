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
    pub fn command_is_valid(){

    }
    pub fn route_command(&mut self, input:String){
       let  command_vec =  input.as_str().split(" ");
       if self.command_is_valid(command_vec) == true {

       }
       else{
           
       }

    }


}