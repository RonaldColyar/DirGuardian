
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
        file_extension : &mut Queue<char>,
        old_name : &str ,status:&mut bool){

    let dot :char = ".".chars().next().unwrap();
    //reverse iteration add into the queue
    for letter in old_name.chars().rev(){
        file_extension.add(letter);
        if letter == dot {
            *status = true;
            break;
        }
    }

 }
 fn file_name_with_suffix_and_ext(old_name:&str , file_extension:  &mut Queue<char>)->String{
    let mut new_name:String  = old_name
            .chars() 
            .take(old_name.len() - file_extension.size())
            .collect(); //name without the extension
    new_name = new_name + "Protected"; //suffix
    for n in 1..file_extension.size(){
        new_name.push(file_extension.remove().unwrap());
    }
    return new_name;
 }
 fn encrypted_file_name(old_name: &str) -> String{
    let mut file_extension : Queue<char>  = queue![];
    let mut status = false;
    encryptor::find_extension(
        &mut file_extension,
        old_name,
        &mut status);
    if status == false{
        return old_name.to_owned() + "Protected";
    }
    else{
        return encryptor::file_name_with_suffix_and_ext(old_name,&mut file_extension );
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