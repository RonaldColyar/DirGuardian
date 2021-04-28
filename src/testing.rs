
use crate::encryptor;
use crate::logger;
use crate::sockethandler;
use std::mem;
use std::net::{Shutdown, TcpStream};
pub struct tester{

}

impl tester{
    pub fn test_decrypted_name(){
        let mut  ok =vec![]; 
        let data = "txt.";
        for i in data.chars(){
            ok.push(i);
        }
        println!("ronprotected ->{}" ,  encryptor::encryptor::file_name_without_suffix("ronprotected",&mut vec![]));
        println!("ron -> {}" ,  encryptor::encryptor::file_name_without_suffix("ron",&mut vec![]));
        println!(" ron.txt -> {}" ,  encryptor::encryptor::file_name_without_suffix("ron.txt" , &mut ok));
        println!(" ronprotected.txt -> {}" ,  encryptor::encryptor::file_name_without_suffix("ronprotected.txt" , &mut ok));
    }


    pub fn test_logger_increment(){
        let test_val1:&i32 = &8 ;
        let test_val2 : &i32 = &9;
        let mut logger = logger::Logger::new(8);
        assert_eq!(&logger.row_number ,test_val1 );
        logger.unknown_command();
        assert_eq!(&logger.row_number , test_val2);
    }
    //the tests below must be configured with middle ware
    // call the mirroring functions in middle.py'


    pub fn test_connection(){
        let mut handler = sockethandler::SockHandler::new();
        let result = handler.connect();
        assert_eq!(result ,true);
        handler.sock.unwrap().shutdown(Shutdown::Both);
    }

    pub fn test_basic_json_request(){
        let mut handler = sockethandler::SockHandler::new();
        handler.connect();
        let mut data_holder = json::JsonValue::new_object();
        data_holder["password"] = "test".into();
        data_holder["command"] = "test".into();
        let stringified_json =  data_holder.dump();
        let response = handler.send_request_and_gather_response(stringified_json);
        println!("Expected Success ,Middleware responded with :{}" , response);

    }
}