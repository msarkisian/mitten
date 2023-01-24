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
