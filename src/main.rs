mod config;
mod github;
mod fib;
mod pr_parser;

fn main() {
    let config = config::Config::new();

    println!("Enable Fibonacci Calculation: {}", config.enable_fib);
    println!("Max Threshold: {}", config.max_threshold);

    if config.enable_fib {
        println!("Fibonacci calculation is enabled.");
    } else {
        println!("Fibonacci calculation is disabled.");
    }

    println!("Max Fibonacci threshold is set to {}", config.max_threshold);

    let pr_content = github::fetch_pr_content();
    let numbers = pr_parser::extract_numbers(&pr_content);
    println!("Extracted numbers: {:?}", numbers);

    let fib_numbers: Vec<i32> = numbers
        .iter()
        .filter(|&&n| n <= config.max_threshold)
        .map(|&n| fib::calculate(n))
        .collect();
    println!("Fibonacci numbers: {:?}", fib_numbers);

    if let Err(e) = github::post_to_github(&fib_numbers) {
        eprintln!("Error posting to GitHub: {}", e);
    }
}
