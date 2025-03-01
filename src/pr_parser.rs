pub fn extract_numbers_from_text(text: String, max_threshold: u32) -> Vec<u32> {
    let mut numbers = Vec::new();
    for word in text.split_whitespace() {
        if let Ok(num) = word.parse::<u32>() {
            if num <= max_threshold {
                numbers.push(num);
            }
        }
    }
    numbers
}

pub fn parse_bool(input: &str) -> Option<bool> {
    match input {
        "1" | "true" => Some(true),
        "0" | "false" => Some(false),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_numbers_from_text() {
        let text = "Here are some numbers: 5, 8, 13, and 21!".to_string();
        assert_eq!(extract_numbers_from_text(text, 20), vec![5, 8, 13]);
    }

    #[test]
    fn test_parse_bool() {
        assert_eq!(parse_bool("1"), Some(true));
        assert_eq!(parse_bool("true"), Some(true));
        assert_eq!(parse_bool("0"), Some(false));
        assert_eq!(parse_bool("false"), Some(false));
        assert_eq!(parse_bool("invalid"), None);
    }
}
