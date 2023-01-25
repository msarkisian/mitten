use rayon::prelude::*;
use std::{
    collections::{BinaryHeap, HashMap},
    fs::read_to_string,
    path::Path,
};
#[macro_use]
extern crate lazy_static;

const GLOVE_PATH: &str = "./datasets/glove.pruned.300d.txt";

lazy_static! {
    static ref WORD_VECS: HashMap<String, [f64; 300]> = load_word_vectors(GLOVE_PATH);
}

#[derive(PartialEq)]
pub struct ScoredWord {
    pub word: &'static str,
    pub score: f64,
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

pub enum WordOp {
    Addition,
    Subtraction,
}

pub struct WordArithmeticOps<'a>(pub &'a str, pub WordOp);

fn load_word_vectors<P: AsRef<Path>>(path: P) -> HashMap<String, [f64; 300]> {
    let words = read_to_string(path).unwrap();
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

pub fn find_closest_words(target: &str) -> Option<BinaryHeap<ScoredWord>> {
    let this_word_vec = get_vector(&target)?;
    Some(find_closest_words_from_vec(this_word_vec))
}

fn find_closest_words_from_vec(input: &[f64; 300]) -> BinaryHeap<ScoredWord> {
    WORD_VECS
        .par_iter()
        .map(|(word, vec)| ScoredWord {
            word,
            score: dot_product(input, vec),
        })
        .collect()
}

pub fn word_arithmetic(initial: &str, ops: &[WordArithmeticOps]) -> Option<&'static str> {
    let mut word_vec = get_vector(initial)?.clone();
    for WordArithmeticOps(word, op) in ops {
        let op_word_vec = get_vector(word)?;
        match op {
            WordOp::Addition => word_vec = add_vecs(&word_vec, &op_word_vec),
            WordOp::Subtraction => word_vec = sub_vecs(&word_vec, &op_word_vec),
        }
    }
    Some(closest_word_from_vec(&word_vec))
}

fn dot_product<const S: usize>(a: &[f64; S], b: &[f64; S]) -> f64 {
    a.iter()
        .zip(b.iter())
        .map(|(a, b)| a * b)
        .fold(0f64, |acc, prod| acc + prod)
}

fn add_vecs<const S: usize>(a: &[f64; S], b: &[f64; S]) -> [f64; S] {
    let mut arr = [0f64; S];
    for i in 0..S {
        arr[i] = a[i] + b[i]
    }
    arr
}

fn sub_vecs<const S: usize>(a: &[f64; S], b: &[f64; S]) -> [f64; S] {
    let mut arr = [0f64; S];
    for i in 0..S {
        arr[i] = a[i] - b[i]
    }
    arr
}

pub fn closest_word(word: &str) -> &'static str {
    find_closest_words(&word).unwrap().pop().unwrap().word
}

fn closest_word_from_vec(input: &[f64; 300]) -> &'static str {
    find_closest_words_from_vec(input).pop().unwrap().word
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
