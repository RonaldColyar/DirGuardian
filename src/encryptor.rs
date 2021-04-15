struct encryptor;

impl encryptor{


 pub fn encrypt(key:str , output_pathing : str , input_pathing:str){   

    let mut encrypt_config = ef::Config::new();
    encrypt_config.input_stream(ef::InputStream::File(input_pathing))
    .output_stream(ef::OutputStream::File(output_pathing))
    .add_output_option(ef::OutputOption::AllowOverwrite)
    .initialization_vector(ef::InitializationVector::GenerateFromRng)
    .password(ef::PasswordType::Text(key, ef::scrypt_defaults()))
    .encrypt();

 }
}

