// use std::arch::x86_64::_mm256_mask_cvtusepi64_storeu_epi8;
use std::f32::consts::E;
use std::net::{TcpListener , TcpStream};
use std::io::prelude::*;
fn main() {
    let listerner = TcpListener::bind("127.0.0.1:7878").unwrap(); 

    // let incoming_stream = listerner.incoming() ; 
    for incoming_stream in listerner.incoming() {
        
        match incoming_stream {

            Ok(incoming_stream) => {
                println!("Connection Established");
                handle_connection(incoming_stream);
            }
            Err(e) => {
                println!("Erro is Connection Establishing {:?} ", e); 
            }
            
        }
        
    }

}

fn handle_connection(mut stream : TcpStream  )  {
    let mut buffer = [0; 1024]; 
    stream.read(&mut buffer).unwrap();

    println!(
        "Request {}" , 
        String::from_utf8_lossy(&buffer[..])
    );
    

    let response = "HTTP/1.1 200 OK r\n\r\n";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
 