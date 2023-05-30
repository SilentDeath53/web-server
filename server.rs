use hyper::{Body, Request, Response, Server};
use hyper::rt::Future;
use hyper::service::service_fn_ok;

fn main() {
    let handle_request = || {
        service_fn_ok(|req: Request<Body>| {
            let response = match req.uri().path() {
                "/" => Response::new(Body::from("Hello, World!")),
                "/about" => Response::new(Body::from("About page")),
                _ => Response::builder()
                    .status(404)
                    .body(Body::from("Not Found"))
                    .unwrap(),
            };

            response
        })
    };

    let addr = ([127, 0, 0, 1], 3000).into();
    let server = Server::bind(&addr)
        .serve(handle_request)
        .map_err(|e| eprintln!("server error: {}", e));

    println!("Server running at http://localhost:3000");

    hyper::rt::run(server);
}
