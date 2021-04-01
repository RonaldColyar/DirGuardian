#[path = "logger.rs"]
mod logger;


use std::io::stdin;
pub struct Interface{
    running : bool,
    color_prefs : [String;3],
    logger : logger::Logger
}

impl Interface{
    pub fn new() -> Self{
        Self{
            running :true,
            color_prefs : ["1".to_owned() , "2".to_owned(),"3".to_owned()],
            logger : logger::Logger::new(0)
        }
    }
    pub fn start(&mut self ){
        self.logger.first_start_message();
        while self.running == true {


            let mut input : String = String::new();
             stdin().read_line(&mut input);
        }
    }

}