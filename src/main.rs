

extern crate json;
mod sockethandler;
mod logger;
mod interface;
mod router;
mod setup;
mod encryptor;



use rand::seq::SliceRandom;



fn main(){

    let mut running = true;
    let mut ui = interface::Interface::new();
    print!("\x1B[2J\x1B[1;1H");  //clears console

    
    ui.router.logger.first_start_message();
    ui.start();
    
 
    
}