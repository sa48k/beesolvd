
use std::env;
// use std::fs::File;
// use std::io::{BufRead, BufReader, Write};

fn main() {
    // get input file path from command line args
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Pass one argument of seven letters, e.g. 'abcdefg'");
    }

    let mut letters = [' '; 7];
    let chars_iter = args[1].chars();

    for (i, c) in chars_iter.enumerate() {
        if i < letters.len() {
            letters[i] = c;
        } else {
            panic!("Too many characters in argument");
        }
    }

    println!("{:#?}", letters);

    // Read the wordlist.txt file
    
    // Loop through word list and produce a new list of valid words.
    
    // Disregard any that don't contain the center letter

    // or contain any letters that AREN'T among those seven

    
}

struct Letters {
    centre_letter: char,
    outside_letters: [char; 6],
}

fn parse_args(centre: &String, outsides: &String) {
    let centre_letter = match centre.chars().next() {
        Some(c) => c,
        None => panic!("Error: No centre letter"),
    };

    println!("{}", centre_letter);

    let mut outside_letters = [' '; 6];
    for (i, c) in outsides.chars().enumerate() {
        outside_letters[i] = c;
    }
    
    println!("Searching for words that can be spelled using {} and MUST contain the letters in {}", centre_letter, outsides);

    Letters {
        centre_letter,
        outside_letters
    };
}

// fn find_words(letters: Letters) {

// }