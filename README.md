[![Version](https://img.shields.io/crates/v/html_url_scraper)](https://crates.io/crates/html_url_scraper)
[![License: Apache 2.0](https://img.shields.io/badge/license-Apache%202.0-red)](https://github.com/pchchv/html_url_scraper/blob/main/LICENSE)
[![Downloads](https://img.shields.io/crates/d/html_url_scraper)](https://crates.io/crates/html_url_scraper)

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