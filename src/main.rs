#![allow(warnings, unused)]
extern crate encryptfile;
extern crate json;
mod sockethandler;
use std::io::stdin;
use std::io::Write;

mod encryptor;
mod interface;
mod jsonhandler;
mod logger;
mod router;
mod setup;
mod testing;
use std::env;
extern crate uuid;

fn main() {
    testing::tester::test_logger_increment();
    let path_en1 = env::current_dir().unwrap().to_str().unwrap().to_owned() + "/src/" + "test/";
    let mut router = router::Router::new(1, 2);
    println!("{}", path_en1);
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    stdin().read_line(&mut input);
    input = input + " ";
    router.route_command(input.as_str());

    //encryptor::encryptor::decrypt_dir_and_sub_dirs(path_en1.as_str(),key);
}
