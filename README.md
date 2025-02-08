# html-url-scraper

Simple library for quickly fetching a list of URLs from a webpage.

# Example

```rust
use html_url_scraper::UrlScraper;

fn main() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("Failed to create runtime");

    rt.block_on(async {
        let scraper = match UrlScraper::new("https://google.com").await {
            Ok(scraper) => scraper,
            Err(e) => {
                eprintln!("Error: {}", e);
                return;
            }
        };

        for (text, url) in scraper.into_iter() {
            println!("Текст: {}, Url: {}", text, url);
        }
    });
}
```