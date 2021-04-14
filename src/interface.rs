


use crate::router::Router;
use std::io::stdin;
use std::io::Write;





pub struct Interface{
    running : bool,
    color_prefs : [String;3],
    pub router : Router,

}

impl Interface{
    pub fn new() -> Self{
        Self{
            running :true,
            color_prefs : ["1".to_owned() , "2".to_owned(),"3".to_owned()],
            router: Router::new(0,0),

        }
    }

    pub fn start(&mut self ){
        while self.running == true {

            let mut input : String = String::new();
            
            std::io::stdout().flush().unwrap();
            stdin().read_line(&mut input);
            self.router.route_command(input);
        }
    }

}