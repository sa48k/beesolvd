use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn solve(letters: [char; 7]) -> Result<Vec<String>, std::io::Error> {
    let letter_str: String = letters.iter().collect();
    println!("Searching for words that can be made using the letters {} (must contain {})", letter_str.to_uppercase(), letters[0].to_uppercase());

    // Read the wordlist.txt file
    let file = File::open("wordlist.txt")?;
    let reader = BufReader::new(file);
    let mut answers = Vec::new();

    // Iterate through lines, check to see if we can make that word
    for file_line in reader.lines() {
        let line = match file_line {
            Ok(l) => l,
            Err(e) => {
                eprintln!("Error reading line: {}", e);
                continue;
            }
        };
        if is_valid_word(&line, letters) {
            answers.push(line);
        }
    }

    // Sort answers vector by length (longest words last) and return it
    answers.sort_by_key(|word| word.len());
    Ok(answers)
}

fn main() -> Result<(), std::io::Error> {
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

    let answers = solve(letters)?;
    println!("{:#?}", answers);
    Ok(())
}

// Takes a word and the char array and checks to see if the word can be 
// made from the letters in the array. It MUST contain the first (centre) letter
fn is_valid_word(word: &str, letters: [char; 7]) -> bool {
    word.trim().chars().all(|c| letters.contains(&c)) && word.contains([letters[0]])
}