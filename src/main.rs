
extern crate encryptfile;
extern crate json;
mod sockethandler;
mod logger;
mod interface;
mod router;
mod setup;
mod encryptor;
mod jsonhandler;
use std::env;

use rand::seq::SliceRandom;
#[macro_use]
extern crate queues;

use queues::*;


fn main(){

    let mut running = true;
    let mut ui = interface::Interface::new();
    print!("\x1B[2J\x1B[1;1H");  //clears console
    let path = 
    env::current_dir()
    .unwrap()
    .to_str()
    .unwrap()
    .to_owned() + "/src/test1.txt";
    let path2 = 
    env::current_dir()
    .unwrap()
    .to_str()
    .unwrap()
    .to_owned() + "/src/test2.txt";
 
    let result = encryptor::encryptor::decrypt("iloveyou" ,&path2 ,&path  );
    if result.is_ok(){
        println!("works well");
    }
    
    ui.router.logger.first_start_message();
    ui.start();
   

    
}