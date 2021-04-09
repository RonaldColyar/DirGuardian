
#[path = "router.rs"]
mod router;
#[path = "setup.rs"]
mod setup;

use std::io::stdin;
use std::io::Write;
pub struct Interface{
    running : bool,
    color_prefs : [String;3],
    pub router : router::Router,
    setup_obj : setup::SetupObj
}

impl Interface{
    pub fn new() -> Self{
        Self{
            running :true,
            color_prefs : ["1".to_owned() , "2".to_owned(),"3".to_owned()],
            router: router::Router::new(0,0),
            setup_obj : setup::SetupObj::new()
        }
    }

    pub fn start(&mut self ){
        while self.running == true {

            let mut input : String = String::new();
            print!(">");
            std::io::stdout().flush().unwrap();
            stdin().read_line(&mut input);
            self.router.route_command(input);
        }
    }

}