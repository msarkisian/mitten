use mitten::{self, context::GuessResult, random_contexto};
use std::{io, process};

fn main() {
    // loop {
    //     let mut word = String::new();
    //     io::stdin().read_line(&mut word).unwrap();
    //     if let Some(closest_words) = mitten::calculate_word_distance_from_word(&word.trim()) {
    //         closest_words
    //             .into_iter_sorted()
    //             .take(50)
    //             .for_each(|w| println!("{}: {}", w.word, w.score));
    //         continue;
    //     }
    //     eprintln!("provided word not in dataset")
    // }
    let game = random_contexto();
    let mut best_guess = None;
    let mut best_distance = usize::MAX;

    loop {
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).unwrap();
        match game.guess(&guess.trim()) {
            GuessResult::Correct => {
                println!("Word guessed correctly!");
                process::exit(0);
            }
            GuessResult::Incorrect { distance: None } => {
                println!("Guessed word not in dataset");
            }
            GuessResult::Incorrect {
                distance: Some(dist),
            } => {
                println!("{}", dist);
                if dist < best_distance {
                    best_guess = Some(guess.trim().to_string());
                    best_distance = dist;
                }
                println!(
                    "Best guess so far: {}, {}",
                    best_guess.as_ref().unwrap(),
                    best_distance
                );
            }
        }
    }
}
