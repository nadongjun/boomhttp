#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(util::generate_response("404", "ERROR"), "HTTP/1.1 404 ERROR\r\nContent-Length: 0\r\n\r\n");
    }
}