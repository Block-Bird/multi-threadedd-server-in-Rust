// use std::arch::x86_64::_mm256_mask_cvtusepi64_storeu_epi8;
use std::f32::consts::E;
use std::fs::{*, self};
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
    
    let get = b"GET / HTTP/1.1\r\n"; 

    if buffer.starts_with(get) {

        println!("Printing If Condition");

        let contents: String = fs::read_to_string("index.html").unwrap(); 
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}", 
            contents.len(), 
            contents
        );
    
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();

    }
    else {

        println!("Printing Else Condition");
        let content = fs::read_to_string("404.html").unwrap(); 

        let response = format!(
            "HTTP/1.1 404 NOT FOUND\r\nContent-Length: {}\r\n\r\n{}", 
            content.len(), 
            content
        );

        stream.write(response.as_bytes()).unwrap(); 

    }

}
 