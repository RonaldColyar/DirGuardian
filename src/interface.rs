use crate::router::Router;
use std::io::stdin;
use std::io::Write;
use std::env;

pub struct Interface{
    running : bool,
    pub router : Router,
}

impl Interface{
    pub fn new() -> Self{
        Self{
            running :true,
            router: Router::new(0,0),
        }
    }
    
    pub fn clear_terminal(&mut self){
        print!("\x1B[2J\x1B[1;1H");
    }

    pub fn start(&mut self ){
        while self.running == true {
            let mut input : String = String::new();
            print!("-->");
            std::io::stdout().flush().unwrap();
            stdin().read_line(&mut input);
            if input.trim() == "quit()"{
                break
            }
            else if input.trim() == "clear()"{
                self.clear_terminal()
            }
            else if input.trim() == "help()"{
               self.router.logger.commands();
            }
            else{
                self.router.route_command(input.trim());
                }
            }
        }
    }
