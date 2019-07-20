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
