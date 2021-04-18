
use crate::encryptor;
use crate::logger;
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
}