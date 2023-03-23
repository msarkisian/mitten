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
        let mut rankings_heap = calculate_word_distance_from_word(word).unwrap();
        let mut rankings_map = HashMap::with_capacity(rankings_heap.len());

        let mut idx = 1usize;
        while let Some(word) = rankings_heap.pop() {
            rankings_map.insert(word.word, idx);
            idx += 1;
        }
        Self {
            word,
            rankings: rankings_map,
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
