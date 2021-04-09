



#[path = "logger.rs"]
mod logger;
mod interface;
use rand::seq::SliceRandom;



fn main(){

    let mut running = true;
    let mut ui = interface::Interface::new();
    print!("\x1B[2J\x1B[1;1H");  //clears console

   
    ui.router.logger.first_start_message();
    ui.start();
    
 
    
}