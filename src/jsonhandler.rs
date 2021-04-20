use json;

use std::io::prelude::*;
pub struct JsonHandler {
    file: std::fs::File,
    counter: i8,
}

//hope to have multiple field updates
impl JsonHandler {
    pub fn new(file: std::fs::File) -> Self {
        Self {
            file: file,
            counter: 0,
        }
    }
    pub fn new_json_obj(data: String) -> std::result::Result<json::JsonValue, std::io::Error> {
        let result = json::parse(data.as_str());
        Ok(result.unwrap())
    }

    //takes old json(in file)
    //add changed value
    //returns stringified version
    pub fn new_json_string(&mut self, type_of_config: String, value: String) -> String {
        let mut content_string = String::new();
        self.file.read_to_string(&mut content_string);
        let result = json::parse(&content_string.as_str());
        if result.is_ok() {
            let mut data = result.unwrap();
            data[type_of_config] = value.as_str().into();
            return data.dump();
        } else {
            return "issue".to_owned();
        }
    }
}
