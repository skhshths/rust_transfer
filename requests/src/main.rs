use std::io;
use serde::Deserialize;

#[derive(Deserialize)]
struct Fact {
    text: String,
    source: String,
}

impl Fact {
    fn show(&self) {
        println!("{} || {}", self.text, self.source);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::builder()
        .user_agent("fetcher")
        .build()?;
    
    let mut url = String::new();
    io::stdin().read_line(&mut url);
    
    let fact: serde_json::Value = client
        .get(url.trim())
        .send()?
        .error_for_status()?
        .json()?;
    
    println!("{fact:#}");
    Ok(())
}