extern crate futures;
extern crate gotham;
extern crate hyper;
extern crate mime;

use futures::{future, Future, Stream};
use hyper::{Body, StatusCode};

use gotham::state::{State, FromState};
use gotham::handler::{HandlerFuture, IntoHandlerError};
use gotham::router::builder::{build_simple_router, DefineSingleRoute, DrawRoutes};
use gotham::helpers::http::response::create_response;
use gotham::router::Router;

const WELCOME: &'static str = "Listening for POSTs to echo";

pub fn get_handler(state: State) -> (State, &'static str) {
    (state, WELCOME)
}

pub fn post_handler(mut state: State) -> Box<HandlerFuture> {
    let f = Body::take_from(&mut state)
        .concat2()
        .then(|full_body| match full_body {
            Ok(valid_body) => {
                let res = create_response(
                    &state, 
                    StatusCode::OK,
                    mime::TEXT_PLAIN,
                    valid_body
                );
                future::ok((state, res))
            }
            Err(e) => future::err((state, e.into_handler_error())),
        });
    
    Box::new(f)
}

fn router() -> Router {
    build_simple_router(|route| {
        route.associate("/", |assoc| {
            assoc.get().to(get_handler);
            assoc.post().to(post_handler);
        });
    })
}

fn main() {
    let addr = "0.0.0.0:8080";
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, router())
}

#[cfg(test)]
mod tests {
    use super::*;
    use gotham::test::TestServer;
    use hyper::StatusCode;

    #[test]
    fn receive_listening_response() {
        let test_server = TestServer::new(router()).unwrap();
        let response = test_server
            .client()
            .get("http://localhost:8080")
            .perform()
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.read_body().unwrap();
        assert_eq!(&body[..], b"Listening for POSTs to echo");
    }

    #[test]
    fn post_request() {
        let test_server = TestServer::new(router()).unwrap();
        let response = test_server
            .client()
            .post("http://localhost:8080", "communism will win", mime::TEXT_PLAIN)
            .perform()
            .unwrap();
        
        assert_eq!(response.status(), StatusCode::OK);

        let body = response.read_body().unwrap();
        assert_eq!(&body[..], b"communism will win");
    }
}