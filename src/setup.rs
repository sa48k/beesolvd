// Reads a text file (one word per line). Write all words 
// that are four letters or longer to wordlist.txt.

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

fn main() {
    // get input file path from command line args
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    
    // Read the words_alpha file
    println!("Loading file: {file_path}...");
    let file = File::open(file_path).expect("Error opening the file");
    let reader = BufReader::new(file);

    // open the output file for writing to
    let output_path = "wordlist.txt";
    let mut output_file = File::create(output_path).expect("Error creating the output file");

    // iterate over lines in the input file
    for line in reader.lines() {
        let word = line.expect("Error reading a line");
        // write 4+ letter words to file
        if word.chars().count() >= 4 {
            writeln!(output_file, "{}", word).expect("Error while writing.");
        }
    }

    println!("DONE");
}