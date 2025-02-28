use std::env;

pub struct Config {
    pub enable_fib: bool,
    pub max_threshold: i32,
}

impl Config {
    pub fn new() -> Self {
        let enable_fib = env::var("INPUT_ENABLE_FIB").unwrap_or_else(|_| "false".to_string()) == "true";
        let max_threshold = env::var("INPUT_MAX_THRESHOLD").unwrap_or_else(|_| "1000".to_string()).parse().unwrap_or(1000);

        Config { enable_fib, max_threshold }
    }
}
