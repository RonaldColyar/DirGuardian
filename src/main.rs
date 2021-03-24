



mod logger;


fn main(){
    let mut logger = logger::Logger::new(0);
    
    logger.first_start_message();
}