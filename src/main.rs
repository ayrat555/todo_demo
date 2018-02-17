mod config;

extern crate dotenv;

extern crate env_logger;

#[macro_use]
extern crate log;

#[cfg(test)]
extern crate spectral;

use dotenv::dotenv;
use config::Config;


fn main() {
    dotenv().ok();
    env_logger::init();

    let cfg = Config::default();
    info!("using {:?}", cfg);
}
