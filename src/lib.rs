use reqwest::Url;
use scraper::html::Select;

/// Iterator returns `(String, Url)` pairs per iteration.
pub struct UrlIter<'a, 'b> {
    url: &'a Url,
    data: Select<'a, 'b>
}