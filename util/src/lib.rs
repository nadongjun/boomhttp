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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {

        /*let response_code = "404";
        let status = "ERROR";

        let mut contents = String::new();
        let response = format!(
            "HTTP/1.1 {} {}\r\nContent-Length: {}\r\n\r\n{}",
            response_code,
            status,
            contents.len(),
            contents
        );*/
         
        assert_eq!(generate_response("404", "ERROR"), "HTTP/1.1 404 ERROR\r\nContent-Length: 0\r\n\r\n");
    }
}