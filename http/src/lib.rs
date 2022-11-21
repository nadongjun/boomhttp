use std::io::prelude::*;
use std::fs::File;
use std::fs;
use std::net::TcpListener;
use std::net::TcpStream;
use backend;
use pool::ThreadPool;
use std::fs::{OpenOptions};

pub fn init_http_server() {
    let listener = TcpListener::bind("localhost:8080").unwrap();
    let pool = ThreadPool::new(4);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("---- HTTP Connection established! ----\n");

		pool.execute(|| {
			handle_connection(stream);
		});

    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();
    let get = b"GET /"; 
    let del = b"DELETE"; 
    let put = b"PUT"; 
    let post = b"POST"; 

    let http_request_url = std::str::from_utf8(&buffer).unwrap();    
    let split: Vec<&str> = http_request_url.split(" ").collect();
    println!("data : {}",split[1]);
    let mut a: String = split[1].to_string();
    a.remove(0);
    let receive_data: Vec<&str> = a.split("&").collect();

    if buffer.starts_with(get) { // GET
        let mut file = File::open(receive_data[0]).unwrap();

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        println!("data {}", contents);

        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();

    } else if buffer.starts_with(put) { //PUT
        let mut file = match File::create(&receive_data[0]) {
            Err(why) => panic!("couldn't create {}", why),
            Ok(file) => file,
        };
        let mut content_parse = "";
        if receive_data.len() == 2  { // Check
            content_parse = receive_data[1]
        }
        match file.write_all(content_parse.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", content_parse, why),
            Ok(_) => println!("successfully wrote to {}", content_parse),
        }

        stream.write(backend::generate_response("200", "OK").as_bytes()).unwrap();
        stream.flush().unwrap();

    } else if buffer.starts_with(del) { // DELETE
        fs::remove_file(receive_data[0]);
        stream.write(backend::generate_response("200", "OK").as_bytes()).unwrap();
        stream.flush().unwrap();

    } else if buffer.starts_with(post) { // POST

        let mut file = match File::create(&receive_data[0]) {
            Err(why) => panic!("couldn't create {}", why),
            Ok(file) => file,
        };
        let mut content_parse = "";
        if receive_data.len() == 2  { // Check
            content_parse = receive_data[1]
        }
        match file.write_all(content_parse.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", content_parse, why),
            Ok(_) => println!("successfully wrote to {}", content_parse),
        }

        stream.write(backend::generate_response("200", "OK").as_bytes()).unwrap();
        stream.flush().unwrap();

    } else {
        stream.write(backend::generate_response("404", "NOT OK").as_bytes()).unwrap();
        stream.flush().unwrap();
    }
 
}
