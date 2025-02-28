#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_fibonacci() {
        assert_eq!(fib::calculate(0), 0);
        assert_eq!(fib::calculate(1), 1);
        assert_eq!(fib::calculate(2), 1);
        assert_eq!(fib::calculate(3), 2);
        assert_eq!(fib::calculate(5), 5);
        assert_eq!(fib::calculate(10), 55);
    }

    #[test]
    fn test_extract_numbers() {
        let pr_content = "Here are the numbers: 5, 8, 13, and 21";
        let numbers = pr_parser::extract_numbers(pr_content);
        assert_eq!(numbers, vec![5, 8, 13, 21]);
    }

    #[test]
    fn test_post_to_github() {
        env::set_var("GITHUB_TOKEN", "test_token");
        env::set_var("GITHUB_API_URL", "https://api.github.com");
        env::set_var("GITHUB_REPOSITORY", "test/repo");
        env::set_var("GITHUB_REF", "test_ref");

        let fib_numbers = vec![1, 1, 2, 3, 5, 8, 13];

        let result = github::post_to_github(&fib_numbers);

        assert!(result.is_ok());
    }
}
