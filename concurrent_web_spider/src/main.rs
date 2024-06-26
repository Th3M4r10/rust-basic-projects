use reqwest::Error;

#[tokio::main]
async fn main() {
    let urls_to_crawl = vec![
        "https://google.com",
        "https://github.com",
        "https://twitter.com"
        // we can add more if we need
    ];

    fetch_and_process(urls_to_crawl).await;

    println!("All fetches initiated.");
}

async fn fetch_url(url: &str) -> Result<String, Error> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}

fn count_words(html: &str) -> usize {
    html.split_whitespace().count()
}

async fn fetch_and_process(urls: Vec<&str>) {
    let fetches = urls.into_iter().map(|url| {
        async move {
            let html = fetch_url(url).await.unwrap_or_else(|_| String::from("Failed to fetch"));
            let word_count = count_words(&html);
            println!("{} has {} words.", url, word_count);
        }
    });
    futures::future::join_all(fetches).await;
}

