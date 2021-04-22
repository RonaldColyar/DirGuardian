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
    let mut interface = interface::Interface::new();
    interface.clear_terminal();
    interface.router.logger.first_start_message();
    interface.start();

}