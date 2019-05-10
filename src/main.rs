
mod scraper;

fn main() {
    for name in scraper::find_names_in_page() {
        println!("{}", name);
    }
}