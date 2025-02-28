use reqwest::blocking::Client;
use serde_json::Value;
use std::env;

pub fn fetch_pr_body(owner: &str, repo: &str, pr_number: u32) -> Option<String> {
    let token = env::var("GITHUB_TOKEN").ok()?;
    let url = format!(
        "https://api.github.com/repos/{}/{}/pulls/{}",
        owner, repo, pr_number
    );

    let client = Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "FibBot")
        .header("Authorization", format!("token {}", token))
        .send()
        .ok()?
        .json::<Value>()
        .ok()?;

    response["body"].as_str().map(|s| s.to_string())
}
