//! The scraper module is meant to centralize
//! scraping utilities.

use scraper::Html;
use scraper::Selector;

/// Scrap asnc.cam.ac.uk/personalnames for celtic names
/// doing some minor cleanup and collecting the names in
/// a string vector.
pub fn find_names_in_page() -> Vec<String> {
    let uri = "https://www.asnc.cam.ac.uk/personalnames/search.php?s_name=@";
    let response = ureq::get(uri).timeout_connect(10_000).call();
    let page_text = &response.into_string().unwrap();

    let body_document = Html::parse_document(page_text);
    let name_selector = Selector::parse("tr td b a").unwrap();

    body_document
        .select(&name_selector)
        .map(|element| {
            element
                .inner_html()
                .replace("&lt;", "<")
                .replace("&gt;", ">")
                .chars()
                .filter(|c| c.is_alphabetic())
                .map(|c| c.to_ascii_lowercase())
                .collect()
        })
        .collect()
}
