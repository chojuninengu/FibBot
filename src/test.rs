#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(5), 5);
        assert_eq!(fibonacci(10), 55);
    }

    #[test]
    fn test_extract_numbers() {
        let text = "Here are some numbers: 5, 8, 13, and 21!";
        let numbers = extract_numbers(text);
        assert_eq!(numbers, vec![5, 8, 13, 21]);
    }
}
