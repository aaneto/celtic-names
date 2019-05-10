//! The scraper module is meant to centralize
//! scraping utilities.

use scraper::Html;
use scraper::Selector;

/// Scrap asnc.cam.ac.uk/personalnames for celtic names
/// doing some minor cleanup and collecting the names in
/// a string vector.
pub fn find_names_in_page() -> Vec<String> {
    let body = reqwest::get("https://www.asnc.cam.ac.uk/personalnames/search.php?s_name=@")
        .expect("Cannot fetch API")
        .text()
        .expect("Cannot read response text");

    let body_document = Html::parse_document(&body);
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
