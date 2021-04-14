use json;
use json;

struct JsonHandler{
    file : std::fs::File,
    counter: i8
}

//hope to have multiple field updates
impl JsonHandler{

    pub fn new(file : std::fs::File)-> Self{
        Self{
            file : file,
            counter : 0
        }

    } 
    pub fn new_json_string(type_of_config : String , value :String)-> String{


        let result = json::parse(file.read_to_string());
        if result.is_ok(){
            let data = result.unwrap();
            data[type_of_config]  = value;
            return data.dump();
        }
        else{
            return "issue".to_owned();
        }
    }

}