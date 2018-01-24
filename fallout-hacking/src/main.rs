extern crate rand;

use std::io;
use std::io::prelude::*;
use std::fs::File;
use rand::{thread_rng, seq, Rng};

fn get_difficulty() -> usize {
    // can this be turned into a while let?
    let mut difficulty : i32 = 0;
    while difficulty <= 0 || difficulty > 5 {
        let mut input = String::new();
        println!("Difficulty (1-5)? ");
        io::stdin().read_line(&mut input).expect("Error reading stdin");
        match input.trim().parse::<i32>() {
            Ok(v) => {
                difficulty = v;
                if difficulty <= 0 || difficulty > 5 {
                    print!("Input out of range. ");
                }
            },
            Err(_) => print!("Input must be a number. "),
        }
    }
    difficulty as usize
}

// does it make more sense to use &str? having difficulty making it work with the borrow-checker
fn load_words(dict : &Vec<String>, difficulty : usize) -> Vec<&String> {
    let length = 4 + difficulty;
    let count = 3 + 2*difficulty;
    seq::sample_iter(&mut thread_rng(), dict.iter().filter(|w| w.len() == length), count).expect("Not enough words in dictionary satisfying difficulty settings")
}

fn num_matching_chars(secret : &str, guess : &str) -> usize {
    secret.chars().zip(guess.chars()) .fold(0, |acc, (s, g)| acc + (s == g) as usize)
}

fn main() {
    let dict_file = File::open("enable1.txt").expect("Failed to open dictionary");
    let dictionary : Vec<String> = io::BufReader::new(dict_file).lines() .map(|l| l.expect("Error parsing dictionary")).collect();

    let bank = load_words(&dictionary, get_difficulty());
    bank.iter() .for_each(|w| println!("{}", w.to_uppercase()));
    let secret = rand::thread_rng().choose(&bank).expect("Error choosing a word");

    for i in 0..4 {
        println!("Guess ({} left)? ", 4-i);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error reading stdin");

        // secret is lowercase as loaded from file
        let matching = num_matching_chars(secret.as_ref(), input.to_lowercase().as_ref());
        println!("{} matching letters", matching);
        if matching == secret.len() {
            println!("You win!");
            return;
        }
    }
    println!("You lose! The word was {}", secret.to_uppercase());
}