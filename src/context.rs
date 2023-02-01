use std::collections::HashMap;

use crate::calculate_word_distance_from_word;

pub struct Contexto {
    word: &'static str,
    rankings: HashMap<&'static str, usize>,
}

pub enum GuessResult {
    Correct,
    Incorrect { distance: Option<usize> },
}

impl Contexto {
    pub fn new(word: &'static str) -> Self {
        Self {
            word,
            rankings: calculate_word_distance_from_word(word)
                .unwrap()
                .into_iter_sorted()
                .map(|w| w.word)
                .filter(|w| *w != word)
                .zip(1..)
                .collect(),
        }
    }

    pub fn guess(&self, guess: &str) -> GuessResult {
        if guess == self.word {
            return GuessResult::Correct;
        }
        GuessResult::Incorrect {
            distance: self.rankings.get(guess).copied(),
        }
    }
}
