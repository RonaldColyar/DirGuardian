#[path = "logger.rs"]
mod logger;

#[path = "router.rs"]
mod router;

use std::io::stdin;
use std::io::Write;
pub struct Interface{
    running : bool,
    color_prefs : [String;3],
    logger : logger::Logger,
    router : router::Router,
    
}

impl Interface{
    pub fn new() -> Self{
        Self{
            running :true,
            color_prefs : ["1".to_owned() , "2".to_owned(),"3".to_owned()],
            logger : logger::Logger::new(0),
            router: router::Router::new(0,0)
        }
    }

    pub fn start(&mut self ){
        self.logger.first_start_message();
        while self.running == true {

            let mut input : String = String::new();
            print!(">");
            std::io::stdout().flush().unwrap();
            stdin().read_line(&mut input);
            self.router.route_command(input);
        }
    }

}