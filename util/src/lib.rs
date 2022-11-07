pub fn generate_response(response_code : &str,status : &str) -> String {
    let contents = String::new();
    let response = format!(
        "HTTP/1.1 {} {}\r\nContent-Length: {}\r\n\r\n{}",
        response_code,
        status,
        contents.len(),
        contents
    );

    return response;
}

pub fn parse_url(url : &str) -> String {

    
}