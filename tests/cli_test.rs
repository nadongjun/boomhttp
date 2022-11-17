#[cfg(test)]
mod tests {
    #[test]
    fn generate_response_test() {
        assert_eq!(backend::generate_response("404", "ERROR"), "HTTP/1.1 404 ERROR\r\nContent-Length: 0\r\n\r\n");
    }
}