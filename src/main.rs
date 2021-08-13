extern crate rand;
extern crate functions;

use std::fs::read_to_string;
use rand::prelude::*;
use functions::functions::input::str_input;
use std::iter::FromIterator;

fn main() {
    let raw_words = read_to_string("word_list.txt").expect("invalid file");
    let words:Vec<&str> = raw_words.split(',').collect();
    let to_guess:&str = words.choose(&mut rand::thread_rng()).expect("empty vector");
    let mut lives:usize = to_guess.len()+check_complexity(to_guess)-2;
    let word_vector:Vec<char> = to_guess.chars().collect();
    let mut guesses_vec:Vec<char> = vec!['_';word_vector.len()];
    let mut used_chars:Vec<char> = Vec::new();
    let mut won:bool = false;
    println!("You're word is {} and you have {} lives", String::from_iter(&guesses_vec), lives);
    while (!won)&&(lives>0) {
        let guess = str_input("Guess a letter or the entire word");
        let guess_chars:Vec<char> = guess.chars().collect();
        let guess_char = guess_chars[0];

        if (guess.len()==1)&&(used_chars.iter().any(|&x|x==guess_char)) {
            println!("You've already guessed that")
        } else if &guess==to_guess {
            won=true;
            break;
        } else if (word_vector.iter().any(|&i|i==guess_char))&&(guess.len()==1) {
            for i in 0..word_vector.len() {
                if word_vector[i]==guess_char {
                    guesses_vec[i]=word_vector[i];
                }
            }
            if guesses_vec==word_vector {
                won=true;
                break;
            }
        } else {
            println!("nope");
            lives -= 1;
        }

        if (guess.len()==1)&&!(used_chars.iter().any(|&x|x==guess_char)) {used_chars.push(guess_char)}
        println!("You've used:{:?}\nYou're at {} and have {} lives", used_chars, String::from_iter(&guesses_vec), lives);
    }
    if won {println!("Congratulations on getting it right,\nThe word was {} all along",to_guess);}
    else {println!("Congratulations on failing, the word was {}",to_guess)}
}

fn check_complexity(to_check:&str) -> usize {
    let mut used_letters:Vec<char> = Vec::new();
    for i in to_check.chars() {
        if !used_letters.iter().any(|&x|x==i) {
            used_letters.push(i);
        }
    }
    return used_letters.len();
}