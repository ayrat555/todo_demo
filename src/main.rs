mod config;

extern crate dotenv;

extern crate env_logger;

#[macro_use]
extern crate log;
extern crate hyper;
extern crate futures;

#[cfg(test)]
extern crate spectral;

use dotenv::dotenv;
use config::Config;
use hyper::{Body, Response};
use hyper::header::ContentType;
use hyper::server::{Http, const_service, service_fn};


fn main() {
    dotenv().ok();
    env_logger::init();

    let cfg = Config::default();
    info!("using {:?}", cfg);

    let http_addr = ([0,0,0,0], cfg.http_port).into();

    let new_service = const_service(service_fn(|_| {
        Ok(Response::<Body>::new()
           .with_header(ContentType::plaintext())
           .with_body("hello world"))
    }));

    let http_src = Http::new().bind(&http_addr, new_service).unwrap();
    http_src.run().unwrap();
}
