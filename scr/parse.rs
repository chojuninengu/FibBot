fn extract_numbers(text: &str) -> Vec<u32> {
    text.split_whitespace()
        .filter_map(|word| word.parse::<u32>().ok())
        .collect()
}

fn main() {
    let pr_text = "Here are some numbers: 5, 8, and 13.";
    let numbers = extract_numbers(pr_text);
    println!("Extracted numbers: {:?}", numbers);
}
