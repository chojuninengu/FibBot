pub fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

pub fn get_fibonacci_results(numbers: &[u32]) -> Vec<(u32, u32)> {
    numbers.iter().map(|&n| (n, fibonacci(n))).collect()
}
