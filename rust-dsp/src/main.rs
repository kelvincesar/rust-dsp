
use actix_web::{web, App, HttpServer};

mod signals {
    pub mod transform;
    pub mod basics;
}

mod routes {
    pub mod transform;
}

use signals::transform::fft;
use routes::transform::{route_transform_fft};

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        App::new()
            .service(route_transform_fft)

    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
// https://dev.to/dbanty/replacing-fastapi-with-rust-part-3-trying-actix-32lp
// https://github.com/kelvincesar/rust_learning/blob/cefb2b0197062e5f42715fd519018c7dfc20fbd1/rust_book/ch11_tests/adder/src/lib.rs