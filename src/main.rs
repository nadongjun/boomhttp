use parser;
use http;

fn main() {
    parser::init_parser();
    http::init_http_server();
}