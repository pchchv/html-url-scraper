use reqwest::Url;
use url::ParseError;
use scraper::html::Select;

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