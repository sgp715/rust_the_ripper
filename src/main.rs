/**
 * rust the ripper
 *
 * This is a rust implementation of the password cracker John the Ripper.
 *
 * Usage: cargo run <password_hashes> <wordlist>
 *
 */


#[macro_use]
extern crate clap;
use clap::App;
extern crate blake2;
extern crate cracker;
use cracker::Cracker;
use std::fs::File;

use std::io::{BufRead, BufReader};

extern crate num_cpus;

mod mangle;
use mangle::letter_replace;


fn main() {

    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let hash_file = match File::open(matches.value_of("HASHES").unwrap()) {
        Ok(file) => file,
        Err(e) => {
            println!("Error opening password hashes: {}", e);
            return
        },
    };

    let hashes_vec: Vec<String> = BufReader::new(hash_file).lines()
                            .map(|l| l.expect("Error reading hashlist")).collect();

    // let wordlist_file = match File::open(matches.value_of("WORDLIST").unwrap()) {
    //     Ok(file) => file,
    //     Err(e) => {
    //         println!("Error opening wordlist: {}", e);
    //         return
    //     },
    // };

    let mut wordlist_vec = vec![];
    let wordlist_option = matches.value_of("WORDLIST");
    if wordlist_option.is_some() {
        let wordlist_file = match File::open(wordlist_option.unwrap()) {
            Ok(file) => file,
            Err(e) => {
                println!("Error opening wordlist: {}", e);
                return
            },
        };
        wordlist_vec = BufReader::new(wordlist_file).lines()
                                    .map(|l| l.expect("Error reading wordlist")).collect();
    } else {
        wordlist_vec.push("".to_string());
    }

    let number_threads = match matches.value_of("THREADS") {
        Some(num) => {
            num.parse::<usize>().unwrap()
        },
        _ => num_cpus::get()
    };

    let cracker = Cracker::new(hashes_vec, wordlist_vec, "password.pot".to_string());
    cracker.crack(number_threads, letter_replace);

}
