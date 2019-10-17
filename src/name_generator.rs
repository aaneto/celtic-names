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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_custom_markov() {
        let mut boxed_markov = markov::Chain::new();

        let name_generator: &mut dyn NameGenerator = &mut boxed_markov as &mut dyn NameGenerator;
        name_generator.feed("aabdc".to_string());

        let mut markov = markov::Chain::new();
        markov.feed("aabdc".chars().collect());

        let name_size = 4;

        let first_name = name_generator.generate(name_size);
        let second_name: String = markov.generate().into_iter().take(name_size).collect();

        assert_eq!(boxed_markov, markov);
        // Names are random, just assert they at least have the same length
        assert_eq!(first_name.len(), second_name.len());
    }

    #[test]
    fn test_crate_markov() {
        let arbitrary_markov_order = 3;
        let mut boxed_markov = markov_chain::MarkovChain::new(arbitrary_markov_order);

        let name_generator: &mut dyn NameGenerator = &mut boxed_markov as &mut dyn NameGenerator;
        name_generator.feed("aabdc".to_string());

        let mut markov = markov_chain::MarkovChain::new(3);
        markov.feed("aabdc".to_string());

        let name_size = 4;

        let first_name = name_generator.generate(name_size);
        let second_name: String = markov.generate_str(name_size);

        assert_eq!(boxed_markov, markov);
        // Names are random, just assert they at least have the same length
        assert_eq!(first_name.len(), second_name.len());
    }
}
