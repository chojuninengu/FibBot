use octocrab::Octocrab;
use regex::Regex;
use std::error::Error;
use tokio;

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
        let mut a = 0;
        let mut b = 1;
        for _ in 2..=n {
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
        let re = Regex::new(r"\d+").unwrap();
        re.find_iter(pr_content)
            .filter_map(|digits| digits.as_str().parse::<u32>().ok())
            .collect()
    }
}

// Module that interacts with GitHub API
mod github {
    use octocrab::Octocrab;
    use std::error::Error;
    use crate::pr_parser::extract_numbers;

    pub async fn get_pr(pr_number: u64) -> Result<Vec<u32>, Box<dyn Error>> {
        let octocrab = Octocrab::default();

        let files = octocrab
            .pulls("chojuninengu", "FibBot") // Update with your repo
            .list_files(pr_number)
            .await?; // Await the result

        let files = files.items.first().and_then(|f| f.patch.clone()).unwrap_or_default();

        println!("Pull Request Contents:\n{}", files);

        let nums = extract_numbers(&files);
        println!("Collected Nums: {nums:?}");

        Ok(nums)
    }

    pub async fn post_comment(response: &str) -> Result<(), Box<dyn Error>> {
        println!("Posting comment:\n{}", response);
        Ok(())
    }
}

use config::Config;
use fib::fibonacci;
use github::{get_pr, post_comment};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load configuration
    let config = Config::new();

    println!("FibBot application is running...");
    println!("Fibonacci Calculation Enabled: {}", config.enable_fib);
    println!("Max Threshold is: {}", config.max_threshold);

    // Define a pull request number (should be dynamically fetched)
    let pr_number = 1; // Change this accordingly

    // Fetch PR numbers
    let pr_numbers = get_pr(pr_number).await?;

    if pr_numbers.is_empty() {
        println!("No numbers found in this pull request.");
        return Ok(());
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

    // Post the results as a comment on the pull request
    if let Err(e) = post_comment(&response).await {
        eprintln!("Error posting comment: {}", e);
    }

    Ok(())
}
