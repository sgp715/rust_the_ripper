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

use std::io::Read;

extern crate blake2;
use blake2::{Blake2b, Digest};

fn main() {

    if  env::args().len() == 3 {

        let mut hash_file = match File::open(env::args().nth(1).expect("Could not get hash file name")) {
            Ok(file) => file,
            Err(e) => {
                println!("Error opening password hashes: {}", e);
                return
            }
        };

        let mut wordlist_file = match File::open(env::args().nth(2).expect("Could not get wordlist file name")) {
            Ok(file) => file,
            Err(e) => {
                println!("Error opening wordlist: {}", e);
                return
            }
        };

        let mut hashes_string = String::new();
        hash_file.read_to_string(&mut hashes_string);

        let mut wordlist_string = String::new();
        wordlist_file.read_to_string(&mut wordlist_string);


        let cracked = crack(hashes_string.split('\n').collect(), wordlist_string.split('\n').collect());
        // write cracked passwords to file

    } else {
        println!("Usage: cargo run <password_hashes> <wordlist>");
        return
    }

}

fn crack(wordlist: Vec<&str>, hashes: Vec<&str>) -> Vec<(String, String)> {

    let mut generated: Vec<(String, String)> = vec![];
    for word in wordlist {
        let mut hasher = Blake2b::default();
        hasher.input(word.to_string().as_bytes());
        let hash_string = hasher.result().iter().cloned().collect();
        generated.push((word.to_string(), String::from_utf8(hash_string).expect("Could not create string from bytes")));
    }

    let mut passwords: Vec<(String, String)> = vec![];
    'hash: for hash in hashes {
        for g in &generated {
            if g.1 == hash {
                passwords.push((hash.to_string(), g.0.clone()));
                continue 'hash;
            }
        }
    }

    passwords

}
