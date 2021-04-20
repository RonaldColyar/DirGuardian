use ansi_term::Colour;
use std::env;
use std::fs::File;
use std::io::Read;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub struct Logger {
    pub row_number: i32,
}

impl Logger {
    pub fn new(init_num: i32) -> Logger {
        Logger {
            row_number: init_num,
        }
    }

    pub fn invalid_key_size(&mut self) {
        self.display_row_number();
        println!("Key file not valid , the length of the key in the file is 0");
        self.display_bottom_sep();
    }
    pub fn log_encryption_key(&mut self, key: &str) {
        self.display_row_number();
        let path =
            env::current_dir().unwrap().to_str().unwrap().to_owned() + "/src/warningascii.txt";
        let mut f = File::open(path);
        self.file_read_check(f);
        println!("This is your encryption key!!:{}", key);
        println!("You will need this key to decrypt your files!");
        println!("Do not close this directory unless you have this key!!");
        self.display_bottom_sep();
    }
    pub fn invalid_key_path(&mut self) {
        self.display_row_number();
        println!("Key file not found");
        self.display_bottom_sep();
    }
    pub fn unknown_command(&mut self) {
        self.display_row_number();
        println!("Unknown Command!! ");
        self.display_bottom_sep();
    }

    fn display_row_number(&mut self) {
        println!("------------Row Number:{}------------", self.row_number);
        self.row_number += 1;
    }
    fn display_bottom_sep(&mut self) {
        println!("-----------------------------------");
        println!("   ");
    }
    pub fn encrypt_success_message(&mut self, dir: &str) {
        self.display_row_number();
        println!("   ");
        println!(
            "Directory ({}) {} Encrypted",
            Colour::Red.paint(dir),
            Colour::Green.paint("Successfully")
        );
        self.display_bottom_sep();
    }
    pub fn file_not_found(&mut self, path: &str) {
        self.display_row_number();
        println!("Can't find file at {}", path);
        println!("please try again!!");
        self.display_bottom_sep();
    }
    fn welcome(&mut self) {
        println!("   ");
        println!(
            "{} || {}",
            Colour::Green.paint("Welcome To DirGuardian!"),
            Colour::Red.paint("Version ".to_owned() + VERSION)
        );
        println!("  ");
        println!("      For command listings run --Coms");
    }

    fn file_read_check(&mut self, f: std::result::Result<File, std::io::Error>) {
        if f.is_ok() == true {
            let mut data = String::new();
            f.unwrap().read_to_string(&mut data);
            println!("{}", data);
        } else {
            println!(
                "{} {}",
                Colour::Red.paint(
                    "
               
              No Ascii Art Support!! For more information, Please Check: 
              
              https://github.com/RonaldColyar/DirGuardian
              
              "
                ),
                f.unwrap_err()
            )
        }
    }

    pub fn first_start_message(&mut self) {
        let path = env::current_dir().unwrap().to_str().unwrap().to_owned() + "/src/ascii.txt";

        println!("{}", path);
        let mut f = File::open(path);

        self.welcome();
        self.file_read_check(f);
    }
}
