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
        let re = Regex::new(r"\d+").unwrap(); // Matches one or more digits
        re.find_iter(pr_content)
            .filter_map(|digits| digits.as_str().parse::<u32>().ok())
            .collect()
    }
}

// Module that simulates posting a comment to a pull request
mod github {
    use std::error::Error;

    pub async fn post_comment(response: &str) -> Result<(), Box<dyn Error>> {
        // Simulate posting the comment (replace with actual API interaction)
        println!("Posting comment:\n{}", response);
        Ok(())
    }
}

use config::Config;
use fib::fibonacci;
use pr_parser::extract_numbers;
use github::post_comment;

#[tokio::main]
async fn main() {
    // Load configuration
    let config = Config::new();

    println!("FibBot application is running...");
    println!("Fibonacci Calculation Enabled: {}", config.enable_fib);
    println!("Max Threshold is: {}", config.max_threshold);

    // Simulate pulling pull request content (replace with actual GitHub PR fetching logic).
    let pr_content = "Here are the numbers: 5, 8, 13, 21"; // Sample PR content.
    let pr_numbers = extract_numbers(pr_content);

    if pr_numbers.is_empty() {
        println!("No numbers found in this pull request.");
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
}
