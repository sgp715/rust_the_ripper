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

extern crate num_cpus;

fn dumby_mangler(word: String) -> Vec<String> {
    vec![word]
}

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

    let wordlist_file = match File::open(matches.value_of("WORDLIST").unwrap()) {
        Ok(file) => file,
        Err(e) => {
            println!("Error opening wordlist: {}", e);
            return
        },
    };

    let number_threads = match matches.value_of("THREADS") {
        Some(num) => {
            num.parse::<usize>().unwrap()
        },
        _ => num_cpus::get()
    };

    let cracker = Cracker::new(hash_file, wordlist_file, "password.pot".to_string());
    cracker.run(number_threads, dumby_mangler);
}
