
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
use uuid::Uuid;

fn main(){
    let mut uuid = Uuid::new_v4().to_string();
    uuid = uuid +  &Uuid::new_v4().to_string();
    let mut running = true;
    let mut ui = interface::Interface::new();
    print!("\x1B[2J\x1B[1;1H");  //clears console
    println!("{}",uuid);
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
    .to_owned() + "/src/ok.txt";
 
    let result = encryptor::encryptor::encrypt(uuid.as_str(),&path ,&path2  );

    
    if result.is_ok(){
        println!("works well");
    }
    else{
        println!("LO" );
    }

    ui.router.logger.first_start_message();
    ui.start();
   

    
}