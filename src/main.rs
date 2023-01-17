use salvo::prelude::*;

#[handler]
async fn hello_world() -> &'static str {
    "Hello world"
}

#[handler]
async fn service() -> &'static str {
    "Service"
}

#[handler]
async fn about() -> &'static str {
    "About"
}

#[tokio::main]
async fn main() {
    let router = Router::new()
        .push(Router::new().path("").get(hello_world))
        .push(Router::new().path("about").get(about))
        .push(Router::new().path("service").get(service));

    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .serve(router)
        .await
}
