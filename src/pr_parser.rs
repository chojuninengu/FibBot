pub fn extract_numbers(content: &str) -> Vec<i32> {
    content
        .split_whitespace()
        .filter_map(|word| word.parse::<i32>().ok())
        .collect()
}
