// use std::arch::x86_64::_mm256_mask_cvtusepi64_storeu_epi8;
use std::f32::consts::E;
use std::fs::{self, *};
use std::io::prelude::*;
use std::thread::*;
use std::thread;
use std::time::Duration;

// custom defiend Module
use wevServer::ThreadPool;

use std::net::{TcpListener, TcpStream};
 fn main() {

    let listerner = TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = ThreadPool::new(4);

    // let incoming_stream = listerner.incoming() ;
    for incoming_stream in listerner.incoming() {
        
        let incoming_stream = incoming_stream.unwrap(); 

        pool.execute( || {
            handle_connection(incoming_stream); 
        })
        
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, file_name) = 
    
    if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "index.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "index.html")
    }

     else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents: String = fs::read_to_string(file_name).unwrap();

    let response = format!(
        "{}\r\n Content-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    ); 
                
    //         contents.len(),
    //         contents
    //     );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}
