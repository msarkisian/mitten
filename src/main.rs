use std::fs::read_to_string;

fn main() {
    println!("{:?}", get_vector("rust"));
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
}
