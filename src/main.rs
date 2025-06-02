use reqwest::header::{ACCEPT, USER_AGENT};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let owner = "santoshxshrestha";
    let repo = "nexish";

    // GitHub API endpoint
    let url = format!("https://api.github.com/repos/{owner}/{repo}/commits?per_page=1");

    let client = reqwest::Client::new();
    let res = client
        .get(&url)
        .header(USER_AGENT, "rust-lang reqwest")
        .header(ACCEPT, "application/vnd.github.v3+json")
        .send()
        .await?;

    if !res.status().is_success() {
        eprintln!("Failed to fetch commits: {}", res.status());
        return Ok(());
    }

    // GitHub returns pagination info in `Link` header
    if let Some(link_header) = res.headers().get("link") {
        let link_str = link_header.to_str()?;
        // Find the last page URL
        for part in link_str.split(',') {
            if part.contains("rel=\"last\"") {
                // extract the `page=` parameter
                let start = part.find("page=").unwrap() + 5;
                let end = part[start..].find('>').unwrap() + start;
                let total_commits: usize = part[start..end].parse()?;
                println!("Total commits: {}", total_commits);
                return Ok(());
            }
        }
    }

    println!("Only one page of commits found (1 commit)");
    Ok(())
}
