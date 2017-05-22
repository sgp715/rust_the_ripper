use std::fs::File;
use std::io::{BufRead, BufReader};

extern crate blake2;
use blake2::{Blake2b, Digest};

pub struct Cracker {
    hash_file: File,
    wordlist_file: File,
}

impl Cracker {
    pub fn new(h_file: File, w_file: File) -> Self {
        Cracker {
            hash_file: h_file,
            wordlist_file: w_file
        }
    }

    pub fn run(&self) {
        let h_file_clone = match self.hash_file.try_clone() {
            Ok(clone) => clone,
            _ => panic!("Error"),
        };
        let w_file_clone = match self.wordlist_file.try_clone() {
            Ok(clone) => clone,
            _ => panic!("Error"),
        };

        let mut hashes = BufReader::new(h_file_clone).lines();
        let wordlist: Vec<String> = BufReader::new(w_file_clone).lines()
                                    .map(|l| l.expect("Error reading wordlist")).collect();

        while let Some(Ok(hash)) = hashes.next() {
            match self.crack(&hash, &wordlist) {
                Some(word) => println!("Successfully Cracked:\n\tpassword: {}\n\thash: {}\n", word, &hash),
                None => println!("Could not crack hash: {}\n", &hash),
            }
        }
    }

    fn crack(&self, hash: &String, wordlist: &Vec<String>) -> Option<String> {

        for word in wordlist {
            let mut hasher = Blake2b::default();
            hasher.input(word.to_string().as_bytes());
            let hashed_word: Vec<u8> = hasher.result().iter().cloned().collect();
            let mut compare = String::new();
            for byte in hashed_word {
                compare.push_str(format!("{:x}", byte).as_str());
            }
            if *hash.to_lowercase() == compare {
                return Some(word.to_string())
            }
        }

        None
    }
}
