use scraper::Html;
use scraper::Selector;

pub fn find_names_in_page() -> Vec<String> {
    let body = reqwest::get("https://www.asnc.cam.ac.uk/personalnames/search.php?s_name=@")
        .expect("Cannot fetch API")
        .text()
        .expect("Cannot read response text");

    let body_document = Html::parse_document(&body);
    let name_selector = Selector::parse("tr td b a").unwrap();

    body_document.select(&name_selector).map(|element| {
        element
            .inner_html()
            .replace("&lt;", "<")
            .replace("&gt;", ">")
            .chars()
            .filter(|c| c.is_alphabetic())
            .collect()
    }).collect()
}