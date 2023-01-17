use salvo::{prelude::*, serve_static::StaticDir};

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
    let router = Router::with_path("<**path>").get(
        StaticDir::new(["public"])
            .with_defaults("index.html")
            .with_listing(true),
    );

    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .serve(router)
        .await
}
