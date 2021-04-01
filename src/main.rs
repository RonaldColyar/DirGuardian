



mod logger;
mod interface;

fn main(){
   
    let mut running = true;
    let mut ui = interface::Interface::new();
    print!("\x1B[2J\x1B[1;1H");  //clears console
    ui.start();
  
 
    
}