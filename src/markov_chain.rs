use std::collections::HashMap;

use rand::distributions::WeightedIndex;
use rand::prelude::*;

/// The MarkovChain of order N is a mapping
/// from a sequence of tokens to a probability
/// map over multiple possible tokens.
#[derive(Debug, PartialEq, Eq)]
pub struct MarkovChain {
    order: usize,
    transitions: HashMap<Word, ValueFrequency>,
}

impl MarkovChain {
    pub fn new(order: usize) -> Self {
        Self {
            order,
            transitions: HashMap::new(),
        }
    }

    pub fn feed_str<S: ToString>(&mut self, text: S) {
        let text_as_string = text.to_string();

        if text_as_string.len() > self.order {
            let text_chars: Vec<char> = text_as_string.chars().collect();

            for window in text_chars.windows(self.order + 1) {
                let last_char = window[window.len() - 1];
                let word = Word::from_char_slice(&window[0..window.len() - 1]);

                let value_freq = self
                    .transitions
                    .entry(word)
                    .or_insert_with(ValueFrequency::new);

                value_freq.insert(last_char);
            }
        }

        for (_, value_freq) in self.transitions.iter_mut() {
            value_freq.sort();
        }
    }

    pub fn generate_str(&self, size: usize) -> String {
        let mut name = self.pick_random_word();
        let mut rng = rand::thread_rng();

        while name.len() < size {
            let word_string: String = name.chars().skip(name.len() - self.order).take(3).collect();

            let word_window = Word::from_str(&word_string);

            if let Some(value_freq) = self.transitions.get(&word_window) {
                let weights = value_freq.get_weights();
                let letters = value_freq.get_letters();

                let dist = WeightedIndex::new(&weights).unwrap();

                let next_word = letters[dist.sample(&mut rng)];

                name.push(next_word);
            } else {
                break;
            }
        }

        name.chars()
            .enumerate()
            .map(|(i, letter)| {
                if i == 0 {
                    letter.to_ascii_uppercase()
                } else {
                    letter
                }
            })
            .collect()
    }

    fn pick_random_word(&self) -> String {
        let mut rng = rand::thread_rng();

        let keys: Vec<&Word> = self.transitions.keys().collect();
        let index: usize = rng.gen_range(0, keys.len());

        keys[index].data.iter().collect()
    }
}

impl std::fmt::Display for MarkovChain {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut display_str = String::new();

        for (word, value_freq) in self.transitions.iter() {
            let word_str: String = word.data.iter().collect();

            display_str.push_str(&word_str);
            display_str.push_str(": \n");

            for (letter, freq) in value_freq.data.iter() {
                display_str.push_str(&format!("\t{} => {}\n", letter, freq));
            }

            display_str.push_str("\n");
        }

        write!(f, "{}", display_str)
    }
}

/// The word is a slice of a character with
/// some logic.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Word {
    pub data: Vec<char>,
}

impl Word {
    pub fn from_char_slice(slice: &[char]) -> Self {
        Self {
            data: slice.to_vec(),
        }
    }

    pub fn from_str(text: &str) -> Self {
        Self {
            data: text.chars().collect(),
        }
    }
}

/// The weighted result is a struct
/// containing all the possible values
/// of a word mapping with their respective
/// count.
#[derive(Debug, PartialEq, Eq)]
pub struct ValueFrequency {
    data: Vec<(char, usize)>,
}

impl ValueFrequency {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn insert(&mut self, new_letter: char) {
        let mut found = false;

        for (letter, count) in self.data.iter_mut() {
            if *letter == new_letter {
                *count += 1;
                found = true;
            }
        }

        if !found {
            self.data.push((new_letter, 1));
        }
    }

    pub fn sort(&mut self) {
        self.data.sort_by(|fst, snd| snd.1.cmp(&fst.1));
    }

    pub fn get_weights(&self) -> Vec<usize> {
        self.data.iter().map(|(_, count)| *count).collect()
    }

    pub fn get_letters(&self) -> Vec<char> {
        self.data.iter().map(|(letter, _)| *letter).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_text_generation() {
        let mut markov_chain = MarkovChain::new(2);
        markov_chain.feed_str("abab");
        let text_size = 10;
        let random_text = markov_chain.generate_str(text_size);

        // The name is random we just assert it is of the right size
        assert_eq!(random_text.len(), text_size);
    }

    #[test]
    fn test_random_word() {
        let mut markov_chain = MarkovChain::new(2);
        markov_chain.feed_str("aaaa");

        let a_word = markov_chain.pick_random_word();

        assert_eq!(a_word, "aa");
    }

    #[test]
    fn test_markov_chain_display() {
        let mut markov_chain = MarkovChain::new(1);
        markov_chain.feed_str("aaab");
        let mut buffer = Vec::new();

        writeln!(&mut buffer, "{}", markov_chain).unwrap();

        let display_text: String = String::from_utf8(buffer).unwrap();
        let expected_output = "a: \n\ta => 2\n\tb => 1\n\n\n";

        assert_eq!(display_text, expected_output.to_string())
    }

    #[test]
    fn test_word() {
        let word_from_str = Word::from_str("abcdee");
        let word_from_slice = Word::from_char_slice(&['a', 'b', 'c', 'd', 'e', 'e']);

        assert_eq!(word_from_slice, word_from_str);
    }

    #[test]
    fn test_value_frequency() {
        let mut value_freq = ValueFrequency::new();

        for chr in vec!['a', 'a', 'v', 'b', 'b', 'c', 'd'] {
            value_freq.insert(chr);
        }

        value_freq.sort();

        assert_eq!(value_freq.get_letters(), vec!['a', 'b', 'v', 'c', 'd']);
        assert_eq!(value_freq.get_weights(), vec![2, 2, 1, 1, 1]);
    }
}
