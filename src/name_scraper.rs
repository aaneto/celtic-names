//! The scraper module is meant to centralize
//! scraping utilities.

use scraper::Html;
use scraper::Selector;
use std::time::Duration;

///
/// doing some minor cleanup and collecting the names in
/// a string vector.
pub fn find_names_in_page() -> Option<Vec<String>> {
    let page_text_option = fetch_page_text();

    page_text_option.map(find_names_in_html)
}

/// Fetch the text from the asnc.cam.ac.uk/personalnames page,
/// parsing it into a string and returning it.
fn fetch_page_text() -> Option<String> {
    let uri = "https://www.asnc.cam.ac.uk/personalnames/search.php?s_name=@";
    let agent = ureq::AgentBuilder::new()
        .timeout_read(Duration::from_secs(10))
        .timeout_write(Duration::from_secs(10))
        .build();
    let response = agent.get(uri).call().ok()?;

    response.into_string().ok()
}

/// Scrap all celtic names from a HTML page text.
fn find_names_in_html(html_text: String) -> Vec<String> {
    let body_document = Html::parse_document(&html_text);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fetch_names_correctly() {
        let names = find_names_in_page();

        assert!(names.is_some());
    }

    #[test]
    fn parse_valid_html() {
        let page_text = "
        <html>
        <body>
        <table><tbody>
        <tr>
            <td><b><a>Potato</a></b></td>
            <td><b><a>Batata</a></b></td>
            <td><b><a>Wuuur</a></b></td>
        </tr>
        </tbody>
        </table>
        </body>
        </html>
        "
        .to_string();

        let names = find_names_in_html(page_text);

        assert_eq!(
            names,
            vec![
                "potato".to_string(),
                "batata".to_string(),
                "wuuur".to_string()
            ]
        );
    }
}
