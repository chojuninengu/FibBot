use reqwest::blocking::Client;
use serde_json::json;
use std::env;

pub fn fetch_pr_content() -> String {
    "Here are the numbers: 5, 8, 13, 21".to_string()
}

pub fn post_to_github(fib_numbers: &[i32]) -> Result<(), reqwest::Error> {
    let token = env::var("GITHUB_TOKEN").expect("Missing GITHUB_TOKEN environment variable");
    let repo = env::var("GITHUB_REPOSITORY").expect("Missing GITHUB_REPOSITORY environment variable");
    let pr_number = env::var("GITHUB_REF").expect("Missing GITHUB_REF environment variable");
    let api_url = env::var("GITHUB_API_URL").unwrap_or_else(|_| "https://api.github.com".to_string());

    let pr_url = format!("{}/repos/{}/issues/{}/comments", api_url, repo, pr_number);

    let client = Client::new();
    let response = client.post(&pr_url)
        .header("Authorization", format!("Bearer {}", token))
        .json(&json!({
            "body": format!("Fibonacci results: {:?}", fib_numbers),
        }))
        .send();

    match response {
        Ok(res) if res.status().is_success() => {
            println!("Comment posted successfully.");
        }
        Ok(res) => {
            eprintln!("Failed to post comment: {}", res.status());
        }
        Err(e) => {
            eprintln!("Error posting comment: {}", e);
        }
    }

    Ok(())
}
