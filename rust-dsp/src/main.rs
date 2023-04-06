
use actix_web::{web, App, HttpServer};

mod signals {
    pub mod transform;
    pub mod basics;
}

mod routes {
    pub mod transform;
    pub mod index;
}


use routes::transform::{test_fft, pipeline_to_frequency};
use routes::index::{index};



#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(test_fft)
            .service(pipeline_to_frequency)

    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
// https://dev.to/dbanty/replacing-fastapi-with-rust-part-3-trying-actix-32lp
// https://github.com/kelvincesar/rust_learning/blob/cefb2b0197062e5f42715fd519018c7dfc20fbd1/rust_book/ch11_tests/adder/src/lib.rs