
extern crate encryptfile;
extern crate json;
mod sockethandler;
mod logger;
mod interface;
mod router;
mod setup;
mod encryptor;
mod jsonhandler;


use rand::seq::SliceRandom;



fn main(){

    let mut running = true;
    let mut ui = interface::Interface::new();
    print!("\x1B[2J\x1B[1;1H");  //clears console
    let path = 
    env::current_dir()
    .unwrap()
    .to_str()
    .unwrap()
    .to_owned() + "/src/test.txt";
    let path2 = 
    env::current_dir()
    .unwrap()
    .to_str()
    .unwrap()
    .to_owned() + "/src/test1.txt";

    
    ui.router.logger.first_start_message();
    ui.start();
    
    let result = encryptor::encrypt("iloveyou" ,path2 ,path );
    if result.is_ok(){
        println("works well");
    }
    
}