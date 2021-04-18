
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
    testing::tester::test_decrypted_name();
}