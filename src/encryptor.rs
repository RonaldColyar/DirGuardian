
use queues::*;
use crate::encryptfile as ef;
pub struct encryptor;
use std::fs;

impl encryptor{

 fn find_extension( 
        file_extension : &mut Vec<char>,
        old_name : &str ,status:&mut bool){

    let dot :char = ".".chars().next().unwrap();
    //reverse iteration add onto the stack
    for letter in old_name.chars().rev(){
        file_extension.push(letter);
        if letter == dot {
            *status = true;
            break;
        }
    }

 }
 
 fn file_name_with_suffix_and_ext(
     old_name:&str , 
     file_extension:  &mut Vec<char>,
     suffix :&str)->String{
    let mut new_name:String  = old_name
            .chars() 
            .take(old_name.len() - file_extension.len())
            .collect(); //name without the extension
    new_name = new_name +  suffix;
    //add extension back on after adding suffix
    for n in 0..file_extension.len(){
        println!("{}",file_extension.len());
        new_name.push(file_extension.pop().unwrap());
    }
    return new_name;
 }

 pub fn encrypted_file_name(old_name: &str) -> String{
    let mut file_extension : Vec<char> = vec![];
    let mut status = false;
    encryptor::find_extension(
        &mut file_extension,
        old_name,
        &mut status);
    if status == false{
        return old_name.to_owned() + "Protected";
    }
    else{
        return encryptor::file_name_with_suffix_and_ext(old_name,&mut file_extension,"Protected" );
    }
   
    }
//future use
 fn remove_file_if_successful(encrypt_result:&Result<() , ef::EncryptError>  , path:&str){
     //if original encrypted try deleting the original
     if encrypt_result.is_ok(){
         let delete_result = fs::remove_file(path);
         if delete_result.is_ok(){
            println!("File at {} has been successfully encrypted and deleted",path);
         }
     }

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