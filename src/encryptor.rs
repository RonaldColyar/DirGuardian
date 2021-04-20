use crate::encryptfile as ef;
use queues::*;
pub struct encryptor;
use crate::uuid;
use std::fs;

use std::env;

impl encryptor {
    fn find_extension(file_extension: &mut Vec<char>, old_name: &str) {
        let dot: char = ".".chars().next().unwrap();
        //reverse iteration add onto the stack
        for letter in old_name.chars().rev() {
            file_extension.push(letter);
            if letter == dot {
                return;
            }
        }
        // if no dot has been found
        *file_extension = vec![];
    }

    pub fn minimal_encryption_key() -> String {
        let mut base_key = uuid::Uuid::new_v4().to_simple().to_string();
        base_key = base_key + uuid::Uuid::new_v4().to_simple().to_string().as_str();
        return base_key;
    }

    pub fn encrypt_dir(dir_path: &str, key: &str) {
        let entries_result = fs::read_dir(dir_path);
        if entries_result.is_ok() {
            for entry in entries_result.unwrap() {
                encryptor::check_path_and_encrypt(dir_path, entry.unwrap().path(), key);
            }
        }
    }

    // ----------BEGINNING OF  REPEATED LOGIC----------

    /* REPEATED LOGIC FOR  READABILITY */
    pub fn decrypt_dir_and_sub_dirs(dir_path: &str, key: &str) {
        let mut not_found_status = false;
        let mut directory_names = vec![String::from(dir_path)];
        let mut current_index: usize = 0;

        while not_found_status == false {
            //while there are sub directories remaining
            let entries_result = fs::read_dir(directory_names[current_index].to_owned());
            if entries_result.is_ok() {
                //current directory is valid
                for entry in entries_result.unwrap() {
                    encryptor::decrypt_or_add_dirname(
                        entry.unwrap().path(),
                        &mut directory_names,
                        dir_path,
                        current_index,
                        key,
                    );
                }
            } else {
                break; //no initial directory found
            }
            if current_index == directory_names.len() - 1 {
                // there isn't any more sub directories
                not_found_status = true;
            }
            current_index = current_index + 1;
        }
    }
    /* REPEATED LOGIC FOR  READABILITY */
    pub fn encrypt_dir_and_sub_dirs(dir_path: &str, key: &str) {
        let mut not_found_status = false;
        let mut directory_names = vec![String::from(dir_path)];
        let mut current_index: usize = 0;

        while not_found_status == false {
            //while there are sub directories remaining
            let entries_result = fs::read_dir(directory_names[current_index].to_owned());
            if entries_result.is_ok() {
                //current directory is valid
                for entry in entries_result.unwrap() {
                    encryptor::encrypt_or_add_dirname(
                        entry.unwrap().path(),
                        &mut directory_names,
                        dir_path,
                        current_index,
                        key,
                    );
                }
            } else {
                break; //no  directory found
            }
            if current_index == directory_names.len() - 1 {
                // there isn't any more sub directories
                not_found_status = true;
            }
            current_index = current_index + 1;
        }
    }

    /* REPEATED LOGIC FOR  READABILITY */
    fn encrypt_or_add_dirname(
        path: std::path::PathBuf,
        dir_names: &mut Vec<String>,
        dir_path: &str,
        curr_index: usize,
        key: &str,
    ) {
        if path.is_dir() {
            //found a sub directory
            println!("{}", path.to_str().unwrap().to_owned() + "/");
            let name = String::from(path.to_str().unwrap()) + "/";
            dir_names.push(name); // add to list of sub dirs to traverse next
        } else {
            // found a file
            encryptor::check_path_and_encrypt(dir_names[curr_index].as_str(), path, key);
        }
    }
    /* REPEATED LOGIC FOR  READABILITY */
    fn decrypt_or_add_dirname(
        path: std::path::PathBuf,
        dir_names: &mut Vec<String>,
        dir_path: &str,
        curr_index: usize,
        key: &str,
    ) {
        if path.is_dir() {
            //found a sub directory
            println!("{}", path.to_str().unwrap().to_owned() + "/");
            let name = String::from(path.to_str().unwrap()) + "/";
            dir_names.push(name); // add to list of sub dirs to traverse next
        } else {
            // found a file
            encryptor::check_path_and_decrypt(dir_names[curr_index].as_str(), path, key);
        }
    }
    /* REPEATED LOGIC FOR  READABILITY */
    fn check_path_and_encrypt(output_location: &str, path: std::path::PathBuf, key: &str) {
        let name_result = path.file_name();
        if name_result.is_some() {
            println!(" Encrypting->{}", name_result.unwrap().to_str().unwrap());
            let file_name = name_result.unwrap();
            let encrypted_file_name =
                encryptor::plain_encrypted_file_name(file_name.to_str().unwrap());
            let output_file_path =
                String::from(output_location.to_owned()) + encrypted_file_name.as_str();
            encryptor::encrypt("test", output_file_path.as_str(), path.to_str().unwrap());
        }
    }

    /* REPEATED LOGIC FOR  READABILITY */
    fn check_path_and_decrypt(output_location: &str, path: std::path::PathBuf, key: &str) {
        let name_result = path.file_name();
        if name_result.is_some() {
            println!(" Decrypting->{}", name_result.unwrap().to_str().unwrap());
            let file_name = name_result.unwrap();
            let decrypted_file_name =
                encryptor::plain_decrypted_file_name(file_name.to_str().unwrap());
            let output_file_path =
                String::from(output_location.to_owned()) + decrypted_file_name.as_str();
            encryptor::decrypt("test", output_file_path.as_str(), path.to_str().unwrap());
        }
    }
    // ----------ENDING OF REPEATED LOGIC----------

    fn remove_file_if_successful(encrypt_result: &Result<(), ef::EncryptError>, path: &str) {
        //if original encrypted try deleting the original
        if encrypt_result.is_ok() {
            let delete_result = fs::remove_file(path);
            if delete_result.is_ok() {
                println!(
                    "File at {} has been successfully encrypted and deleted",
                    path
                );
            }
        }
    }

    fn gather_correct_decrypt_name_without_ext(old_name: &str, name_holder: &mut String) {
        if old_name.len() > 9 {
            //without suffix
            let name_without_ext_and_suffix: String =
                old_name.chars().take(old_name.len() - 9).collect();
            *name_holder = name_without_ext_and_suffix;
        }
        //no suffix
        else {
            *name_holder = old_name.to_string();
        }
    }
    fn gather_correct_decrypt_name_with_ext(
        old_name: &str,
        mut file_ext: &mut Vec<char>,
        name_holder: &mut String,
    ) {
        let name_without_ext: String = old_name
            .chars()
            .take(old_name.len() - file_ext.len())
            .collect();
        //extension but no suffix
        if name_without_ext.len() <= 9 {
            *name_holder = old_name.to_string();
        }
        //extension and suffix
        else {
            let mut name_without_ext_and_suffix: String = name_without_ext
                .chars()
                .take(name_without_ext.len() - 9)
                .collect();
            encryptor::add_extension(&mut file_ext, &mut name_without_ext_and_suffix);
            *name_holder = name_without_ext_and_suffix; //now has extension
        }
    }

    pub fn file_name_without_suffix(old_name: &str, mut file_ext: &mut Vec<char>) -> String {
        let mut name_holder: String = String::new();
        if file_ext.len() > 0 {
            encryptor::gather_correct_decrypt_name_with_ext(
                old_name,
                &mut file_ext,
                &mut name_holder,
            );
        }
        //no extension
        else {
            encryptor::gather_correct_decrypt_name_without_ext(old_name, &mut name_holder);
        }

        return name_holder;
    }

    fn add_extension(file_ext: &mut Vec<char>, new_name: &mut String) {
        for n in 0..file_ext.len() {
            new_name.push(file_ext.pop().unwrap());
        }
    }

    fn file_name_with_suffix_and_ext(
        old_name: &str,
        mut file_ext: &mut Vec<char>,
        suffix: &str,
    ) -> String {
        let mut new_name: String = old_name
            .chars()
            .take(old_name.len() - file_ext.len())
            .collect(); //name without the extension
        new_name = new_name + suffix;
        encryptor::add_extension(&mut file_ext, &mut new_name);
        return new_name;
    }
    //removes "protected" suffix from plain file name
    fn plain_decrypted_file_name(old_name: &str) -> String {
        let mut file_ext: Vec<char> = vec![];
        encryptor::find_extension(&mut file_ext, old_name);
        return encryptor::file_name_without_suffix(old_name, &mut file_ext);
    }

    pub fn plain_encrypted_file_name(old_name: &str) -> String {
        let mut file_extension: Vec<char> = vec![];
        encryptor::find_extension(&mut file_extension, old_name);
        if file_extension.len() == 0 {
            return old_name.to_owned() + "Protected";
        } else {
            return encryptor::file_name_with_suffix_and_ext(
                old_name,
                &mut file_extension,
                "Protected",
            );
        }
    }

    pub fn encrypt(
        key: &str,
        output_pathing: &str,
        input_pathing: &str,
    ) -> Result<(), ef::EncryptError> {
        let mut encrypt_config = ef::Config::new();
        encrypt_config
            .input_stream(ef::InputStream::File(input_pathing.to_owned()))
            .output_stream(ef::OutputStream::File(output_pathing.to_owned()))
            .add_output_option(ef::OutputOption::AllowOverwrite)
            .initialization_vector(ef::InitializationVector::GenerateFromRng)
            .password(ef::PasswordType::Text(
                key.to_owned(),
                ef::scrypt_defaults(),
            ))
            .encrypt();
        let result = ef::process(&encrypt_config);
        encryptor::remove_file_if_successful(&result, input_pathing);
        return result;
    }
    pub fn decrypt(
        key: &str,
        output_pathing: &str,
        input_pathing: &str,
    ) -> Result<(), ef::EncryptError> {
        let mut decrypt_config = ef::Config::new();
        decrypt_config
            .input_stream(ef::InputStream::File(input_pathing.to_owned()))
            .output_stream(ef::OutputStream::File(output_pathing.to_owned()))
            .add_output_option(ef::OutputOption::AllowOverwrite)
            .password(ef::PasswordType::Text(
                key.to_owned(),
                ef::PasswordKeyGenMethod::ReadFromFile,
            ))
            .decrypt();
        let result = ef::process(&decrypt_config);
        encryptor::remove_file_if_successful(&result, input_pathing);
        return result;
    }
}
