mod fetch_pr;

fn main() {
    let owner = "chojuninengu";  // Replace with actual repo owner
    let repo = "FibBot";         // Replace with actual repo name
    let pr_number = 1;           // Replace with actual PR number

    if let Some(pr_body) = fetch_pr::fetch_pr_body(owner, repo, pr_number) {
        println!("PR Body: {}", pr_body);
    } else {
        println!("Failed to fetch PR body.");
    }
}
