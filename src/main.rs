#![feature(binary_heap_into_iter_sorted)]

use std::{collections::BinaryHeap, fs::read_to_string};

#[derive(PartialEq)]
struct ScoredWord {
    word: String,
    score: f64,
}

impl PartialOrd for ScoredWord {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.score.partial_cmp(&other.score)
    }
}

impl Ord for ScoredWord {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.partial_cmp(&other.score).unwrap()
    }
}

impl Eq for ScoredWord {}

fn main() {
    find_closest_words("cat")
        .into_iter_sorted()
        .take(20)
        .for_each(|w| println!("{}: {}", w.word, w.score))
}

fn get_vector(target: &str) -> Option<[f64; 300]> {
    let words = read_to_string("./glove.840B.300d.txt").unwrap();

    for line in words.lines() {
        let mut tokens = line.split(' ');
        let word = tokens.next().unwrap();
        if word == target {
            let mut word_vec = [0f64; 300];
            for (i, token) in tokens.enumerate() {
                word_vec[i] = token.parse::<f64>().unwrap();
            }
            println!("{:?}", word_vec);
            return Some(word_vec);
        }
    }
    None
}

fn find_closest_words(target: &str) -> BinaryHeap<ScoredWord> {
    let this_word_vec = get_vector(&target).unwrap();
    let mut word_heap = BinaryHeap::new();

    let words = read_to_string("./glove.840B.300d.txt").unwrap();

    for line in words.lines() {
        let mut tokens = line.split(' ');
        let word = tokens.next().unwrap();
        let mut word_vec = [0f64; 300];
        for (i, token) in tokens.enumerate() {
            word_vec[i] = token.parse::<f64>().unwrap();
        }
        word_heap.push(ScoredWord {
            word: word.to_string(),
            score: dot_product(&this_word_vec, &word_vec),
        });
    }
    word_heap
}

fn dot_product<const S: usize>(a: &[f64; S], b: &[f64; S]) -> f64 {
    a.iter()
        .zip(b.iter())
        .map(|(a, b)| a * b)
        .fold(0f64, |acc, prod| acc + prod)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn dot_product_test() {
        assert_eq!(
            dot_product(&[1f64, 3f64, -5f64], &[4f64, -2f64, -1f64]),
            3f64
        );
        assert_eq!(
            dot_product(&[1f64, 3f64, -5f64], &[1f64, 3f64, -5f64]),
            35f64
        );
    }

    #[test]
    fn heap_ordering() {
        let mut heap = BinaryHeap::new();
        heap.push(ScoredWord {
            word: "a".to_string(),
            score: 0f64,
        });
        heap.push(ScoredWord {
            word: "b".to_string(),
            score: 5f64,
        });
        heap.push(ScoredWord {
            word: "c".to_string(),
            score: 2f64,
        });
        heap.push(ScoredWord {
            word: "d".to_string(),
            score: 9f64,
        });
        heap.push(ScoredWord {
            word: "e".to_string(),
            score: 0.030000540530,
        });
        heap.push(ScoredWord {
            word: "f".to_string(),
            score: -0.030000540530,
        });

        assert_eq!(heap.pop().unwrap().score, 9f64);
    }
}
