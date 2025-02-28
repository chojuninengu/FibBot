use reqwest::blocking::Client;
use serde_json::json;

pub fn post_comment(owner: &str, repo: &str, pr_number: u32, token: &str, comment: &str) {
    let url = format!(
        "https://api.github.com/repos/{}/{}/issues/{}/comments",
        owner, repo, pr_number
    );

    let client = Client::new();
    let body = json!({ "body": comment });

    let _ = client
        .post(&url)
        .header("User-Agent", "FibBot")
        .header("Authorization", format!("token {}", token))
        .json(&body)
        .send();
}
