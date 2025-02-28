use regex::Regex;

pub fn extract_numbers_from_body(body: &str) -> Vec<u32> {
    let re = Regex::new(r"\b\d+\b").unwrap();
    re.find_iter(body)
        .filter_map(|mat| mat.as_str().parse::<u32>().ok())
        .collect()
}
