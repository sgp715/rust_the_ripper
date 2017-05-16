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

fn main() {

    let hash_file;
    let wordlist_file;
    // Check for valid command line arguments
    if let Some(arg1) = env::args().nth(1) {
        hash_file = match File::open(arg1) {
            Ok(file) => file,
            Err(e) => {
                println!("Error opening password hashes: {}", e);
                return
            },
        }
    } else {
        println!("Usage: cargo run <password_hashes> <wordlist>");
        return
    }
    if let Some(arg2) = env::args().nth(2) {
        wordlist_file = match File::open(arg2) {
            Ok(file) => file,
            Err(e) => {
                println!("Error opening wordlist: {}", e);
                return
            },
        }
    } else {
        println!("Usage: cargo run <password_hashes> <wordlist>");
        return
    }


   /*   for each hash in hash_file
    *       for each word in word_list
    *           let test = result of running md5 hashing algorithm on word
    *           if test == hash
    *               return word     // we found the password
    */

}
