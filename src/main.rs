mod fib;
mod github_api;
mod test;

use num_bigint::BigUint;
use std::env;
use tokio;

use fib::fibonacci;
use github_api::GhAPIClient;
use test::{extract_numbers_from_text, parse_bool};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {use log::{error, info};
use num_bigint::BigUint;
use std::env;
use tokio;

use fib::fibonacci;
use github_api::GhAPIClient;
use pr_parser::{extract_numbers_from_text, parse_bool};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    info!("Starting FibBot...");

    // Read command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        error!("Mismatch required params, exactly two params are required: enable_fib, max_threshold");
        return Ok(());
    }

    let enable_fib = parse_bool(&args[1].trim()).expect("Could not parse enable_fib from params");
    let max_threshold = args[2].trim().parse().expect("Could not parse max_threshold from params");

    // Read environment variables
    let gh_token = env::var("GITHUB_TOKEN").map_err(|_| "GITHUB_TOKEN not set")?;
    let gh_repo = env::var("GITHUB_REPOSITORY").map_err(|_| "GITHUB_REPOSITORY not set")?;
    let pr_number = env::var("PR_NUMBER").map_err(|_| "PR_NUMBER not set")?;

    let gh_api = GhAPIClient::new(&gh_token, &gh_repo);
    let pr_number = pr_number.parse().expect("Could not parse PR number");

    info!("Fetching pull request files for PR #{}...", pr_number);
    let pr_diff_entry = gh_api.get_pull_request_files(pr_number).await.map_err(|e| {
        error!("Failed to fetch pull request files: {}", e);
        e
    })?;

    let mut numbers = Vec::new();
    for item in pr_diff_entry {
        if let Some(text) = item.patch {
            numbers.append(&mut extract_numbers_from_text(text, max_threshold));
        }
    }

    info!("Numbers from PR content: {:?}", numbers);

    if enable_fib {
        let fibonaccies = numbers
            .iter()
            .map(|num| (*num, fibonacci(*num)))
            .collect::<Vec<(u32, BigUint)>>();

        let comment = if fibonaccies.is_empty() {
            "Numberless PR: Nothing to Compute...".to_string()
        } else {
            format!("Fibonaccies: {:?}", fibonaccies)
        };

        info!("Posting comment: {}", comment);
        gh_api.post_issue_comment(pr_number, &comment).await.map_err(|e| {
            error!("Failed to post comment: {}", e);
            e
        })?;
    } else {
        info!("Fibbot was disabled!");
    }

    Ok(())
}
    let args = env::args().collect::<Vec<String>>();

    if args.len() != 3 {
        panic!("Mismatch required params, exactly two params are required: enable_fib, max_threshold");
    }

    println!("Args: {:?}", args);

    let gh_token = env::var("GITHUB_TOKEN").expect("Could not read GITHUB_TOKEN from env.");
    let gh_repo = env::var("GITHUB_REPOSITORY").expect("Could not read GITHUB_REPOSITORY from env.");
    let pr_number = env::var("PR_NUMBER").expect("Could not read PR_NUMBER from env.");

    let gh_api = GhAPIClient::new(&gh_token, &gh_repo);

    let [_, enable_fib, max_threshold] = args.as_slice() else {
        panic!("Failed to read args");
    };

    let enable_fib = parse_bool(&enable_fib.trim()).expect("Could not parse enable_fib from params");
    let max_threshold = max_threshold.trim().parse().expect("Could not parse max_threshold from params");
    let pr_number = pr_number.parse().expect("Could not parse PR number");

    let pr_diff_entry = gh_api.get_pull_request_files(pr_number).await?;

    let mut numbers = Vec::new();

    for item in pr_diff_entry {
        if let Some(text) = item.patch {
            numbers.append(&mut extract_numbers_from_text(text, max_threshold));
        }
    }

    println!("Numbers from PR content: {:?}", numbers);

    if enable_fib {
        let fibonaccies = numbers
            .iter()
            .map(|num| (*num, fibonacci(*num)))
            .collect::<Vec<(u32, BigUint)>>();

        let comment = if fibonaccies.is_empty() {
            format!("Numberless PR: Nothing to Compute...")
        } else {
            format!("Fibonaccies: {:?}", fibonaccies)
        };

        println!("{comment}");

        // Post response as PR comments
        gh_api.post_issue_comment(pr_number, &comment).await?;
    } else {
        println!("Fibbot was disabled!");
    }

    Ok(())
}
