
use rand::seq::SliceRandom;
pub struct Handler{
     files : [String  ; 5]
}
impl Handler {
    pub fn new() ->Self{
        Self{
            files : [
                "knight".to_owned() ,
                "frog".to_owned() ,
                "monkey".to_owned() ,
                "bat".to_owned() ,
                "elephant".to_owned() ]
        }
    }
    pub fn animal_of_choice(&mut self)-> String{
        let file_name = self.files.choose(&mut rand::thread_rng()).unwrap();
        return file_name.to_owned();
    }
}