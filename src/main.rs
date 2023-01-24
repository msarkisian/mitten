#![feature(binary_heap_into_iter_sorted)]

use std::{
    collections::{BinaryHeap, HashMap},
    fs::read_to_string,
};
#[macro_use]
extern crate lazy_static;

const GLOVE_PATH: &str = "./datasets/glove.pruned.300d.txt";

lazy_static! {
    static ref WORD_VECS: HashMap<String, [f64; 300]> = load_word_vectors();
}

#[derive(PartialEq)]
struct ScoredWord {
    word: &'static str,
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
    find_closest_words("persimmon")
        .into_iter_sorted()
        .take(50)
        .for_each(|w| println!("{}: {}", w.word, w.score))
}

fn load_word_vectors() -> HashMap<String, [f64; 300]> {
    let words = read_to_string(GLOVE_PATH).unwrap();
    let mut word_vectors = HashMap::new();
    for line in words.lines() {
        let mut tokens = line.split(' ');
        let word = tokens.next().unwrap();

        let mut word_vec = [0f64; 300];
        for (i, token) in tokens.enumerate() {
            word_vec[i] = token.parse::<f64>().unwrap();
        }

        word_vectors.insert(word.to_string(), word_vec);
    }
    word_vectors
}

fn get_vector(target: &str) -> Option<&[f64; 300]> {
    WORD_VECS.get(target)
}

fn find_closest_words(target: &str) -> BinaryHeap<ScoredWord> {
    let this_word_vec = get_vector(&target).expect("target word not in dataset");
    let mut word_heap = BinaryHeap::new();

    for (word, vec) in WORD_VECS.iter() {
        word_heap.push(ScoredWord {
            word,
            score: dot_product(this_word_vec, vec),
        })
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
            word: "a",
            score: 0f64,
        });
        heap.push(ScoredWord {
            word: "b",
            score: 5f64,
        });
        heap.push(ScoredWord {
            word: "c",
            score: 2f64,
        });
        heap.push(ScoredWord {
            word: "d",
            score: 9f64,
        });
        heap.push(ScoredWord {
            word: "e",
            score: 0.030000540530,
        });
        heap.push(ScoredWord {
            word: "f",
            score: -0.030000540530,
        });

        assert_eq!(heap.pop().unwrap().score, 9f64);
    }
}
