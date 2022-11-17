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
        when.method(POST).path("/configuration/config.json");
        then.status(200).header("content-type", "config/json").body("test");
    });
    
    let response_post = post(server.url("/configuration/config.json"), "test").unwrap();

    hello_mock.assert();
    assert_eq!(response_post.status(), 200);
}

#[test]
fn put_test() {
    let server = MockServer::start();
    let hello_mock = server.mock(|when, then| {
        when.method(PUT).path("/configuration/config.json");
        then.status(200).header("content-type", "config/json").body("test");
    });
    

    let response_put = put(server.url("/configuration/config.json"), "test").unwrap();

    hello_mock.assert();
    assert_eq!(response_put.status(), 200);
}

#[test]
fn delete_test() {
    let server = MockServer::start();
    let hello_mock = server.mock(|when, then| {
        when.method(DELETE).path("/configuration/config.json");
        then.status(200).header("content-type", "config/json").body("test");
    });
    
    let response_delete = delete(server.url("/configuration/config.json")).unwrap();

    hello_mock.assert();
    assert_eq!(response_delete.status(), 200);
}
