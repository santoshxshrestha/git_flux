use clap::Parser;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, USER_AGENT};
use anyhow::Result;

#[derive(Parser)]
#[command(name = "gitflux")]
#[command(about = "Get quick insights into your GitHub commit activity.", long_about = None)]
struct Cli {
    #[clap(long)]
    token: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("gitflux-cli"));
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("token {:?}", cli.token))?);

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    let resp = client
        .get("https://api.github.com/user")
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    println!("Authenticated as: {}", resp["login"]);

    Ok(())
}
