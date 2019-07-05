//! Definition for the NameGenerator interface, responsible
//! for training itself with input text data and generating
//! output text.
use crate::markov_chain;

pub trait NameGenerator {
    fn feed(&mut self, text: String);
    fn generate(&self, name_size: usize) -> String;
}

impl NameGenerator for markov_chain::MarkovChain {
    fn feed(&mut self, text: String) {
        self.feed_str(text);
    }

    fn generate(&self, name_size: usize) -> String {
        self.generate_str(name_size)
    }
}

impl NameGenerator for markov::Chain<char> {
    fn feed(&mut self, text: String) {
        self.feed(text.chars().collect());
    }

    fn generate(&self, name_size: usize) -> String {
        let mut characters = self.generate();
        characters[0].make_ascii_uppercase();

        characters.into_iter().take(name_size).collect()
    }
}
