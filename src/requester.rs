use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub username: String,
    pub password: String, // TODO: time
}

impl Config {
    pub fn new(username: String, password: String) -> Self {
        Self { username, password }
    }
}

#[derive(Debug)]
pub struct Requester {
    config: Config,
}

impl Requester {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn say_hello(&self) {
        println!("hello!");
    }
}
