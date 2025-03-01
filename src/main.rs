use tokio;
use octocrab::Octocrab;
use std::error::Error;

// Internal configuration module
mod config {
    pub struct Config {
        pub enable_fib: bool,
        pub max_threshold: u32,
    }

    impl Config {
        pub fn new() -> Self {
            Config {
                enable_fib: true,
                max_threshold: 21,
            }
        }
    }
}

// Module that computes Fibonacci numbers
mod fib {
    pub fn fibonacci(n: u32) -> u64 {
        if n == 0 {
            return 0;
        }
        let mut a = 0;
        let mut b = 1;
        for _ in 1..n {
            let temp = a + b;
            a = b;
            b = temp;
        }
        b
    }
}

// Module that extracts numbers from pull request content
mod pr_parser {
    use regex::Regex;
    
    pub fn extract_numbers(pr_content: &str) -> Vec<u32> {
        let re = Regex::new(r"\d+").unwrap(); // Matches one or more digits
        re.find_iter(pr_content)
            .filter_map(|digits| digits.as_str().parse::<u32>().ok())
            .collect()
    }
}

// Module that fetches PR details
mod github {
    use octocrab::Octocrab;
    use std::error::Error;
    
    pub async fn get_pr(pr_number: u64) -> Vec<u32> {
        let octocrab = Octocrab::builder().build().unwrap();
        let files = octocrab
            .pulls("chojuninengu", "FibBot") // Update with your repo
            .list_files(pr_number)
            .await;
        
        let files = match files {
            Ok(result) => result.items,
            Err(e) => {
                eprintln!("Error fetching PR files: {}", e);
                return vec![];
            }
        };
        
        let mut pr_content = String::new();
        for file in files {
            if let Some(patch) = file.patch {
                pr_content.push_str(&patch);
            }
        }
        
        println!("Pull Request Contents:\n{}", pr_content);
        let nums = crate::pr_parser::extract_numbers(&pr_content);
        println!("Collected Nums: {:?}", nums);
        nums
    }

    pub async fn post_comment(owner: &str, repo: &str, pr_number: u64, comment: &str) -> Result<(), Box<dyn Error>> {
        let octocrab = Octocrab::builder().build().unwrap();
        
        let comment_body = serde_json::json!({ "body": comment });
        
        octocrab
            .issues(owner, repo)
            .create_comment(pr_number, &comment_body)
            .await?;
        
        println!("✅ Successfully posted Fibonacci results to PR #{}", pr_number);
        Ok(())
    }
}

use config::Config;
use fib::fibonacci;
use github::{get_pr, post_comment};

#[tokio::main]
async fn main() {
    let config = Config::new();
    let pr_number = 1; // Replace this with the actual PR number dynamically

    println!("Fetching PR content...");
    let pr_numbers = get_pr(pr_number).await;
    
    if pr_numbers.is_empty() {
        println!("No numbers found in this pull request.");
        return;
    }

    let mut response = String::from("#### Fibonacci output of each number in the pull request is:\n");
    for &num in &pr_numbers {
        if num <= config.max_threshold {
            let fib_val = fibonacci(num);
            response.push_str(&format!("- Fibonacci({}) = {}\n", num, fib_val));
        } else {
            response.push_str(&format!("- Skipping Fibonacci({}) due to threshold\n", num));
        }
    }

    // Post comment to the PR
    if let Err(e) = post_comment("chojuninengu", "FibBot", pr_number, &response).await {
        eprintln!("Error posting comment: {}", e);
    } else {
        println!("✅ Comment successfully posted to PR #{}", pr_number);
    }
}
