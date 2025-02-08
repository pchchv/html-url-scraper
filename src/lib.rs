use std::fmt;
use url::ParseError;
use reqwest::{Url, Client};
use scraper::{Html, Selector, html::Select};

/// UrlScraper stores the HTML document in memory.
pub struct UrlScraper {
    url: Url,
    html: Html,
    selector: Selector,
}

impl UrlScraper {
    /// Constructs a new scraper from a given URL.
    pub async fn new(url: &str) -> Result<Self, Error> {
        let client = Client::new();
        Self::new_with_client(url, &client).await
    }

    /// Use an existing `reqwest::Client` to make a request.
    pub async fn new_with_client(url: &str, client: &Client) -> Result<Self, Error> {
        let url = Url::parse(url)?;
        let resp = client.get(url.clone()).send().await?;
        let html = resp.text().await?;

        Ok(Self {
            url,
            html: Html::parse_document(&html),
            selector: Selector::parse("a").expect("failed to create <a> selector"),
        })
    }

    /// In case the HTML has already been fetched in advance,
    /// new_with_html can be used to parse from it directly.
    pub fn new_with_html(url: &str, html: &str) -> Result<Self, Error> {
        Ok(Self {
            url: Url::parse(url)?,
            html: Html::parse_document(html),
            selector: Selector::parse("a").expect("failed to create <a> selector"),
        })
    }

    /// Fetch the URLs using an iterator.
    pub fn into_iter<'a>(&'a self) -> UrlIter<'a, 'a> {
        UrlIter {
            url: &self.url,
            data: self.html.select(&self.selector)
        }
    }
}

/// Iterator returns `(String, Url)` pairs per iteration.
pub struct UrlIter<'a, 'b> {
    url: &'a Url,
    data: Select<'a, 'b>
}

impl<'a, 'b> Iterator for UrlIter<'a, 'b> {
    type Item = (String, Url);

    fn next(&mut self) -> Option<Self::Item> {
        for element in &mut self.data {
            if let Some(url) = element.value().attr("href") {
                if !url.starts_with('?') {
                    if let Ok(url) = self.url.join(url) {
                        return Some((element.inner_html(), url));
                    }
                }
            }
        }
        None
    }
}

#[derive(Debug)]
pub enum Error {
    UrlParsing { why: ParseError },
    Request { why: reqwest::Error }
}

impl From<url::ParseError> for Error {
    fn from(why: url::ParseError) -> Error {
        Error::UrlParsing { why }
    }
}

impl From<reqwest::Error> for Error {
    fn from(why: reqwest::Error) -> Error {
        Error::Request { why }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let error = match *self {
            Error::UrlParsing { ref why } => format!("failed to parse URL: {}", why),
            Error::Request { ref why } => format!("failure in request: {}", why),
        };
        f.write_str(&error)
    }
}