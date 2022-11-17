use httpmock::prelude::*;
use isahc::get;

#[test]
fn simple_test() {
    http::init_http_server();

    let server = MockServer::connect("localhost:8080");

    let hello_mock = server.mock(|when, then|{
        when.path("configuration/config.json");
        then.status(200);
    });

    let response = get(server.url("configuration/config.json")).unwrap();

    hello_mock.assert();
    assert_eq!(response.status(), 200);
}