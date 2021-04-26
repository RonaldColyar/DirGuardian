
use std::io::prelude::*;
use std::net::TcpStream;
use std::option::Option;
use std::result::Result;
use std::str::from_utf8;

pub struct SockHandler{
    sock :Option<TcpStream>

}

impl SockHandler{
    pub  fn new() -> Self{
        Self{
            sock:None
        }
    }

    pub fn connect(&mut self) -> Result<TcpStream , std::io::Error>{
        let result = TcpStream::connect("127.0.0.1:50222")?;
        Ok(result)
    }

    pub fn send_request_and_gather_response(&mut self,data:String) -> String{
        if sock.is_some(){
            let bytes = data.as_bytes();
            let mut  holder = [0 as u8; 1024];
            sock.write(bytes)?;//request
            stream.read(&mut holder);//response

            let decode_result = from_utf8(&holder);
            if decode_result.is_ok(){
                return decode_result.unwrap().to_string();
            }
            else{
                return String::new();
            }
        }
    }

}