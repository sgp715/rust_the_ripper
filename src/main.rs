/**
 * rust the ripper
 *
 * This is a rust implementation of the password cracker John the Ripper.
 *
 * Usage: cargo run <password_hashes> <wordlist>
 *
 */

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

extern crate blake2;
use blake2::{Blake2b, Digest};

fn main() {

    if  env::args().len() == 3 {

        let hash_file = match File::open(env::args().nth(1).expect("Could not get hash file name")) {
            Ok(file) => file,
            Err(e) => {
                println!("Error opening password hashes: {}", e);
                return
            }
        };

        let wordlist_file = match File::open(env::args().nth(2).expect("Could not get wordlist file name")) {
            Ok(file) => file,
            Err(e) => {
                println!("Error opening wordlist: {}", e);
                return
            }
        };

        let mut hashes = BufReader::new(hash_file).lines();
        let wordlist: Vec<String> = BufReader::new(wordlist_file).lines().map(|l| l.expect("Error reading wordlist")).collect();


        while let Some(Ok(hash)) = hashes.next() {
            match crack(&hash, &wordlist) {
                Some(word) => println!("Found Password: {}, hash: {}", word, &hash),
                None => println!("Could not crack hash: {}", &hash),
            }
        }


    } else {
        println!("Usage: cargo run <password_hashes> <wordlist>");
        return
    }

}

fn crack(hash: &String, wordlist: &Vec<String>) -> Option<String> {

    for word in wordlist {
        let mut hasher = Blake2b::default();
        hasher.input(word.to_string().as_bytes());
        let hashed_word: Vec<u8> = hasher.result().iter().cloned().collect();
        let mut compare = String::new();
        for byte in hashed_word {
            compare.push_str(format!("{:x}", byte).as_str());
        }
        if *hash.to_lowercase() == compare {
            println!("Found Password: {}, hash: {}", word, &hash);
            return Some(word.to_string())
        }
    }

    None
}
