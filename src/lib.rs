use std::fmt;
use reqwest::Url;
use url::ParseError;
use scraper::{Html, Selector, html::Select};

/// UrlScraper stores the HTML document in memory.
pub struct UrlScraper {
    url: Url,
    html: Html,
    selector: Selector,
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