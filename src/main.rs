#![allow(warnings, unused)]
extern crate encryptfile;
extern crate json;
mod sockethandler;
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
    testing::tester::test_logger_increment();
    let path_en1 = 
    env::current_dir()
    .unwrap()
    .to_str()
    .unwrap()
    .to_owned() +"/src/"+"test/";
    encryptor::encryptor::encrypt_dir_and_sub_dirs(path_en1.as_str());
}