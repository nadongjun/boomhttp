use cli;
use http;

fn main() {
    cli::init_cli();
    http::init_http_server();
}