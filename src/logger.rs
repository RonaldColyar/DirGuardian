

use ansi_term::Colour;
use std::fs::File;
use std::io::Read;


use std::env;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub struct Logger {
    row_number :  i32

}

impl  Logger {
    pub fn new(init_num :  i32)-> Logger{
         Logger{
            row_number : init_num
        }  
    }
    fn display_row_number(&mut self){
        println!("------------Row Number:{}------------",
            self.row_number    
            );
        self.row_number +=1 ;
    }
    fn display_bottom_sep(&mut self){
        println!("-----------------------------------");
        println!("   ");
    }
    pub fn encrypt_success_message(&mut self,dir:&str){
        self.display_row_number();
        println!("   ");
        println!("Directory ({}) {} Encrypted" ,
            Colour::Red.paint(dir) ,
            Colour::Green.paint("Successfully") );
        self.display_bottom_sep();
        
    }
    pub fn file_not_found(&mut self ,path : &str){
        self.display_row_number();
        println!("Can't find file at {}" ,path);
        println!("please try again!!");
        self.display_bottom_sep();
    }
    fn welcome(&mut self){
        println!("   ");
        println!("{} || {}" , 
            Colour::Green.paint("Welcome To DirGuardian!"),
            Colour::Red.paint("Version ".to_owned() + VERSION));
        println!("  ");
        println!("      For command listings run --Coms");
       
    }
     
    fn file_read_check(
            &mut self ,
            f : std::result::Result<File , 
            std::io::Error>, data: &mut String){
   
           if f.is_ok() == true{
               f.unwrap().read_to_string( data);
               println!("{}" ,data);
               
           }
           else{
               println!("{}" , 
            
              Colour::Red.paint("
               
              No Ascii Art Support!! For more information, Please Check: 
              
              https://github.com/RonaldColyar/DirGuardian
              
              ")
            )
           }
       }
   
   
    
   
    
    pub fn first_start_message(&mut self){
        let mut data = String::new();
        let mut f = File::open("/home/linux/DirGuardian/src/ascii.txt");
        self.welcome();
        self.file_read_check(f,&mut data);
    
       

        
    }

}


