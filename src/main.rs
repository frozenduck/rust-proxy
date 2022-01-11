use hyper::service::{make_service_fn, service_fn};
use hyper::{Client, Server, Request, Response, Body};

async fn echo(request: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    match (request.method(), request.uri().path()) {
        _ => {
            let client = Client::new();

            let mut builder = Request::builder()
                .method(request.method())
                .uri(request.uri());

            let request_headers = request.headers().iter().collect::<Vec<_>>();

            for (key, value) in request_headers {
                builder = builder.header(key, value);
            }

            let req = builder
                .body(request.into_body())
                .expect("req");

            let result = client.request(req).await.unwrap();

            Ok(result)
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = ([127, 0, 0, 1], 8100).into();

    let service = make_service_fn(|_| async { Ok::<_, hyper::Error>(service_fn(echo)) });

    let server = Server::bind(&addr).http1_title_case_headers(true).serve(service);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}
