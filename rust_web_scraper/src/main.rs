use reqwest::Error;
use scraper::{Html, Selector};
    
#[tokio::main]
async fn main() -> Result<(), Error> {
    let body = reqwest::get("https://example.com/quotes").await?.text().await?;
    let fragment = Html::parse_document(&body);
    let quotes = Selector::parse(".quote").unwrap();

    for quote in fragment.select(&quotes) {
        let quote_text = quote.text().collect::<Vec<_>>().join(" ");
        println!("{}", quote_text);
    }

    Ok(())
}