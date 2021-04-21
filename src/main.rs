#![allow(warnings, unused)]
extern crate encryptfile;
extern crate json;
mod sockethandler;
use std::io::stdin;
use std::io::Write;
mod asciiarthandler;
mod logger;
mod interface;
mod router;
mod setup;
mod encryptor;
mod jsonhandler;
mod testing;
use std::env;
extern crate uuid;


fn main(){
 let running  = true;
 let mut router = router::Router::new(1,2);
 router.logger.first_start_message();

 while running == true{
    print!("->");
    std::io::stdout().flush();
    let mut input = String::new();
    stdin().read_line(&mut input);
    let path = 
            env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned() + "/src/test/";
    println!("{}" , path);
            
    if input.trim() == "quit()"{
        break
    }
    else if input.trim() == "help()"{
        router.logger.commands();
    }
    else{
        router.route_command(input.trim());
        }
    }
    //encryptor::encryptor::decrypt_dir_and_sub_dirs(path_en1.as_str(),key);
}