fn main() {
    println!("Hello, world!");
}
fn extract_numbers(text: &str) -> Vec<u64> {
    text.split_whitespace()
        .filter_map(|word| word.parse::<u64>().ok())
        .collect()
}