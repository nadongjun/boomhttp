use std::io::prelude::*;
use std::fs::File;
use std::net::TcpListener;
use std::net::TcpStream;
use util;

pub fn init_http_server() {
    let listener = TcpListener::bind("localhost:8080").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("---- HTTP Connection established! ----\n");
        handle_connection(stream)
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();
    let get = b"GET / HTTP/1.1\r\n";
    let del = b"DELETE / HTTP/1.1\r\n";
    let put = b"PUT / HTTP/1.1\r\n";

    if buffer.starts_with(get) {
        let mut file = File::open("hello.html").unwrap();

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else if buffer.starts_with(put) {
         
        stream.write(util::generate_response("200", "OK").as_bytes()).unwrap();
        stream.flush().unwrap();

    } else if buffer.starts_with(del) {
         stream.write(util::generate_response("200", "OK").as_bytes()).unwrap();
        stream.flush().unwrap();

    } else {
        stream.write(util::generate_response("200", "OK").as_bytes()).unwrap();
        stream.flush().unwrap();
    }
 
}