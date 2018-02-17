use std::env;

const HTTP_PORT_ENV: &str = "HTTP_PORT";
const DEFAULT_HTTP_PORT: &str = "3000";

#[derive(Debug, Clone)]
pub struct Config {
    pub http_port: u16
}

impl Default for Config {
    fn default() -> Self {
        let http_port = env::var(HTTP_PORT_ENV.to_string())
            .unwrap_or(DEFAULT_HTTP_PORT.to_string())
            .parse::<u16>().unwrap();

        Self { http_port }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use spectral::prelude::*;

    #[test]
    fn should_be_default_value() {
        let cfg = Config::default();

        assert_that(&cfg.http_port).is_equal_to(3000);
    }
}
