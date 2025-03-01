use std::env;

pub struct Config {
    pub enable_fib: bool,
    pub max_threshold: u32,
    pub pr_number: u64,
    pub github_token: String,
    pub github_repository: String,
    pub github_api_url: String,
}

impl Config {
    pub fn new() -> Self {
        let enable_fib = env::var("INPUT_ENABLE_FIB")
            .unwrap_or_else(|_| "true".to_string()) == "true";
        let max_threshold = env::var("INPUT_MAX_THRESHOLD")
            .unwrap_or_else(|_| "100".to_string())
            .parse()
            .unwrap_or(100);
        let pr_number = env::var("PR_NUMBER")
            .expect("PR_NUMBER not set")
            .parse::<u64>()
            .expect("Invalid PR_NUMBER");
        let github_token = env::var("INPUT_GITHUB_TOKEN")
            .expect("GITHUB_TOKEN not set");
        let github_repository = env::var("GITHUB_REPOSITORY")
            .expect("GITHUB_REPOSITORY not set");
        let github_api_url = env::var("GITHUB_API_URL")
            .unwrap_or_else(|_| "https://api.github.com".to_string());

        Config {
            enable_fib,
            max_threshold,
            pr_number,
            github_token,
            github_repository,
            github_api_url,
        }
    }
}
