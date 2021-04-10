
use std::io::prelude::*;
use std::net::TcpStream;
use std::option::Option;
use std::result::Result;


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
        let result = TcpStream::connect("12.2.2.2.2")?;
        Ok(result)
    }
    pub fn init_connection(){

    }

}