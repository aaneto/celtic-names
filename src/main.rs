mod markov;
mod scraper;

fn main() {
    let mut chain = markov::MarkovChain::new(3);

    for name in scraper::find_names_in_page() {
        chain.feed_str(name);
    }

    for _ in 0..15 {
        let name = chain.generate_str(7);

        println!("{}", name);
    }
}
