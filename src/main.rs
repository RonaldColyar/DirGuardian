
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
  
    ui.router.logger.first_start_message();
    println!("{}" , encryptor::encryptor::decrypted_file_name("testprotected"));
    ui.start();
   

    
}