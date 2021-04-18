
use crate::sockethandler::SockHandler;

//Handles all communication response/send logic
struct server_interface{
    socket_handler : SockHandler

}

impl server_interface{
    fn init_socket_connection(&mut self) -> bool{
        let result = self.socket_handler.connect();

        if result.is_ok(){
            return true;
        }
        else{
            return false;
        }
    }
}