// /var/www/cyb3ria/src/config.rs
use std::env;

#[derive(Clone)]
pub struct Config {
    pub jwt_secret: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
        }
    }
}