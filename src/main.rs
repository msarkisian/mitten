#![feature(binary_heap_into_iter_sorted)]

use mitten;
use std::io;

fn main() {
    loop {
        let mut word = String::new();
        io::stdin().read_line(&mut word).unwrap();
        if let Some(closest_words) = mitten::calculate_word_distance_from_word(&word.trim()) {
            closest_words
                .into_iter_sorted()
                .take(50)
                .for_each(|w| println!("{}: {}", w.word, w.score));
            continue;
        }
        eprintln!("provided word not in dataset")
    }
}
