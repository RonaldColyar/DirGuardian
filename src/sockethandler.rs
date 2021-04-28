
use std::io::prelude::*;
use std::net::TcpStream;
use std::option::Option;
use std::result::Result;
use std::str::from_utf8;

pub struct SockHandler{
    pub sock :Option<TcpStream> //pub for tests

}

impl SockHandler{
    pub  fn new() -> Self{
        Self{
            sock:None
        }
    }

    pub fn connect(&mut self) -> bool{
        let result = TcpStream::connect("127.0.0.1:50222");
        if result.is_ok(){
            self.sock = Some(result.unwrap());
            return true;
        }
        else{
            return false;
        }
        
    }

    pub fn send_request_and_gather_response(&mut self,data:String) -> String{
        if self.sock.is_some(){
            let bytes = data.as_bytes();
            let mut  holder = [0 as u8; 1024];
            let mut sock_value = self.sock.as_ref().unwrap();
            sock_value.write(bytes);//request
            sock_value.read(&mut holder);//response

            let decode_result = from_utf8(&holder);
            if decode_result.is_ok(){
                return decode_result.unwrap().to_string();
            }
            else{
                return String::new();
            }
        }
        else{
            return String::new();
        }
    }

}