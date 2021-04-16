
use queues::*;
use crate::encryptfile as ef;
pub struct encryptor;

impl encryptor{

 fn decrypted_file_name(old_name : &str) -> String{
    //-9 because our file gets "protected" added
    //when encrypted
    let len = old_name.len();
    let new_name: String = old_name.chars().take(len-9).collect();
    return new_name;
}
 fn find_extension(
        
        counter :&mut i8  , 
        file_extension : &mut Queue<char>,
        old_name : &str ){
    let dot :char = ".".chars().next().unwrap();
    //reverse iteration add into the queue
    //untill dot is found
    for letter in old_name.chars().rev(){
        file_extension.add(letter);
        *counter = *counter +1;
        if letter == dot{
            break;
        }
    }

 }
 fn add_extension(new_name:&mut String , file_extension: &mut Queue<char>){
     //push extension on new file name
    for n in 1..file_extension.size(){
        new_name.push(file_extension.remove().unwrap());
    }
 }
 fn encrypted_file_name(old_name: &str) -> String{
    let mut file_extension : Queue<char>  = queue![];
    let mut counter : i8 = 0;
    encryptor::find_extension(&mut counter,&mut file_extension,old_name);
    let mut new_name:String  = old_name.chars()
        .take(old_name.len() - file_extension.size())
        .collect();
    new_name = new_name + "Protected";
    encryptor::add_extension(&mut new_name , &mut file_extension);
    return new_name;
    }


 pub fn encrypt(key:&str , output_pathing : &str , input_pathing:&str)-> Result<() , ef::EncryptError>{   

    let mut encrypt_config = ef::Config::new();
    encrypt_config.input_stream(ef::InputStream::File(input_pathing.to_owned()))
    .output_stream(ef::OutputStream::File(output_pathing.to_owned()))
    .add_output_option(ef::OutputOption::AllowOverwrite)
    .initialization_vector(ef::InitializationVector::GenerateFromRng)
    .password(ef::PasswordType::Text(key.to_owned(), ef::scrypt_defaults()))
    .encrypt();

    let result = ef::process(&encrypt_config);
    return result;

 }
 pub fn decrypt(key:&str , output_pathing: &str , input_pathing:&str)-> Result<() , ef::EncryptError>{
    let mut decrypt_config = ef::Config::new();
    decrypt_config.input_stream(ef::InputStream::File(input_pathing.to_owned()))
     .output_stream(ef::OutputStream::File(output_pathing.to_owned() ))
     .add_output_option(ef::OutputOption::AllowOverwrite)
     .password(ef::PasswordType::Text(key.to_owned(), ef::PasswordKeyGenMethod::ReadFromFile))
     .decrypt();
     let result = ef::process(&decrypt_config);
     return result;
 }


}