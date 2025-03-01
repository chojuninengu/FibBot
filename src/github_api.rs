use crate::pr_parser::extract_numbers;
use octocrab::Octocrab;
use std::error::Error;

pub async fn get_pr(pr_number: u64) -> Result<Vec<u32>, Box<dyn Error>> {
    // Initialize Octocrab instance
    let octocrab = Octocrab::default();

    // Fetch PR files
    let files = octocrab
        .pulls("chojuninengu", "FibBot")  // Replace with your repo details
        .list_files(pr_number)
        .send()
        .await?;

    // Extract the first file's patch content
    let files = files.items.first().and_then(|f| f.patch.clone()).unwrap_or_default();

    println!("Pull Request Contents:\n{}", files);

    // Extract numbers from the PR patch
    let nums = extract_numbers(&files);
    println!("Collected Nums: {nums:?}");

    Ok(nums)
}
