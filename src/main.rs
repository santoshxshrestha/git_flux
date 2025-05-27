#![allow(unused)]
use clap::Parser;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, USER_AGENT};

pub fn build_github_headers(token: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("token {}", token)).expect("Invalid token"),
    );
    headers.insert(USER_AGENT, HeaderValue::from_static("my-github-cli/0.1"));
    headers
}

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    auth: String,
}

fn main() {
    println!("this is main function");
}
