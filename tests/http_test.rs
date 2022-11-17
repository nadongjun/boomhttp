use httpmock::prelude::*;
use isahc::{get, post, delete, put};

#[test]
fn get_test() {
    let server = MockServer::start();
    let hello_mock = server.mock(|when, then| {
        when.method(GET).path("/configuration/config.json");
        then.status(200).header("content-type", "config/json").body("test");
    });
    

    let response_get = get(server.url("/configuration/config.json")).unwrap();

    hello_mock.assert();
    assert_eq!(response_get.status(), 200);
}

#[test]
fn post_test() {
    let server = MockServer::start();
    let hello_mock = server.mock(|when, then| {
        when.method(GET).path("/configuration/config.json");
        then.status(200).header("content-type", "config/json").body("test");
    });
    

    let response_get = get(server.url("/configuration/config.json")).unwrap();
    /*let response_post = post(server.url(url), "test").unwrap();
    let response_delete = delete(server.url(url)).unwrap();
    let response_put = put(server.url(url), "test").unwrap();*/

    hello_mock.assert();
    assert_eq!(response_get.status(), 200);
}


#[test]
fn put_test() {
    let server = MockServer::start();
    let hello_mock = server.mock(|when, then| {
        when.method(GET).path("/configuration/config.json");
        then.status(200).header("content-type", "config/json").body("test");
    });
    

    let response_get = get(server.url("/configuration/config.json")).unwrap();
    /*let response_post = post(server.url(url), "test").unwrap();
    let response_delete = delete(server.url(url)).unwrap();
    let response_put = put(server.url(url), "test").unwrap();*/

    hello_mock.assert();
    assert_eq!(response_get.status(), 200);
}
