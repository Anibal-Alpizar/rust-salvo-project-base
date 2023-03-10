use rust_embed::RustEmbed;
use salvo::{prelude::*, serve_static::static_embed};

#[handler]
async fn hello_world() -> &'static str {
    "Hello world"
}

#[handler]
async fn service() -> &'static str {
    "Service"
}

#[derive(RustEmbed)]
#[folder = "client/dist"]
struct Assets;

#[handler]
async fn about() -> &'static str {
    "About"
}

#[tokio::main]
async fn main() {
    // let router = Router::new()
    //     .push(Router::new().path("").get(hello_world))
    //     .push(Router::new().path("about").get(about))
    //     .push(Router::new().path("service").get(service))
    //     .push(
    //         Router::with_path("<**path>").get(
    //             StaticDir::new(["public"])
    //                 .with_defaults("index.html")
    //                 .with_listing(true),
    //         ),
    //     );
    let router =
        Router::with_path("<**path>").get(static_embed::<Assets>().with_fallback("index.html"));

    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .serve(router)
        .await
}
