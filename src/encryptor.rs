

use crate::encryptfile as ef;
struct encryptor;

impl encryptor{

// encryptfile::EncryptError for result
//Result<() , EncryptError>
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
 pub fn decrypt(key:&str , output_pathing: &str , input_pathing:&str){
    let mut decrypt_config = ef::Config::new();
    decrypt_config.input_stream(ef::InputStream::File(input_pathing.to_owned()))
     .output_stream(ef::OutputStream::File(output_pathing.to_owned()))
     .add_output_option(ef::OutputOption::AllowOverwrite)
     .password(ef::PasswordType::Text(key.to_owned(), ef::PasswordKeyGenMethod::ReadFromFile))
     .decrypt();

 }
}

