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
use std::collections::HashSet;

use std::io::{BufRead, BufReader};

extern crate num_cpus;

pub fn mangle(word: String)->Vec<String>{

    if word ==  "".to_string(){
        return brute_force();
    }
    else{
        return append_number(word);
    }
}

#[test]
fn transform_test(){
    assert_eq!(mangle("".to_string()), vec!["".to_string()]);
}
pub fn append_number(word: String)->Vec<String>{
    let mut appended = vec![];
    for i in 0..9999 {
        let mut test = format!("{}",word);
        test.push_str(&i.to_string());
        appended.push(test);
    }
    appended
}

pub fn dumby_mangler(word: String) -> Vec<String> {
    vec![word]
}

pub fn letter_replace(word: String)->Vec<String>{
    let mut replaced = HashSet::new();
    replaced.insert(word);

    for w in replaced.clone() {
        replaced.insert(w.replace("i", "1"));
    }
    for w in replaced.clone() {
        replaced.insert(w.replace("e", "3"));
    }
    for w in replaced.clone() {
        replaced.insert(w.replace("o", "0"));
    }
    for w in replaced.clone() {
        replaced.insert(w.replace("b", "8"));
    }
    for w in replaced.clone() {
        replaced.insert(w.replace("s", "$"));
    }
    for w in replaced.clone() {
        replaced.insert(w.replace("a", "4"));
    }
    for w in replaced.clone() {
        replaced.insert(w.replace("z", "2"));
    }

    let result: Vec<String> = replaced.iter().map(|s| s.to_owned()).collect();
    result
}

#[test]
fn letter_replace_test() {
    let word = "letmein".to_string();
    let expected = vec!["letmein".to_string(),
                        "letme1n".to_string(),
                        "l3tm3in".to_string(),
                        "l3tm31n".to_string()];
    let mut actual = letter_replace(word);
    actual.sort_by(|a, b| b.cmp(a));
    assert_eq!(actual, expected);
}

pub fn brute_force()->Vec<String>{
    let mut appended = vec![];
    let chars0 = vec!["".to_string(),"0".to_string(),"1".to_string(),"2".to_string(),"3".to_string(),"4".to_string(),"5".to_string(),"6".to_string(),"7".to_string(),"8".to_string(),"9".to_string(),":".to_string(),";".to_string(),"<".to_string(),"=".to_string(),">".to_string(),"?".to_string(),"@".to_string(),"A".to_string(),"B".to_string(),"C".to_string(),"D".to_string(),"E".to_string(),"F".to_string(),"G".to_string(),"H".to_string(),"I".to_string(),"J".to_string(),"K".to_string(),"L".to_string(),"M".to_string(),"N".to_string(),"O".to_string(),"P".to_string(),"Q".to_string(),"R".to_string(),"S".to_string(),"T".to_string(),"U".to_string(),"V".to_string(),"W".to_string(),"X".to_string(),"Y".to_string(),"Z".to_string(),"a".to_string(),"b".to_string(),"c".to_string(),"d".to_string(),"e".to_string(),"f".to_string(),"g".to_string(),"h".to_string(),"i".to_string(),"j".to_string(),"k".to_string(),"l".to_string(),"m".to_string(),"n".to_string(),"o".to_string(),"p".to_string(),"q".to_string(),"r".to_string(),"s".to_string(),"t".to_string(),"u".to_string(),"v".to_string(),"w".to_string(),"x".to_string(),"y".to_string(),"z".to_string()];
    let chars1 = vec!["".to_string(),"0".to_string(),"1".to_string(),"2".to_string(),"3".to_string(),"4".to_string(),"5".to_string(),"6".to_string(),"7".to_string(),"8".to_string(),"9".to_string(),":".to_string(),";".to_string(),"<".to_string(),"=".to_string(),">".to_string(),"?".to_string(),"@".to_string(),"A".to_string(),"B".to_string(),"C".to_string(),"D".to_string(),"E".to_string(),"F".to_string(),"G".to_string(),"H".to_string(),"I".to_string(),"J".to_string(),"K".to_string(),"L".to_string(),"M".to_string(),"N".to_string(),"O".to_string(),"P".to_string(),"Q".to_string(),"R".to_string(),"S".to_string(),"T".to_string(),"U".to_string(),"V".to_string(),"W".to_string(),"X".to_string(),"Y".to_string(),"Z".to_string(),"a".to_string(),"b".to_string(),"c".to_string(),"d".to_string(),"e".to_string(),"f".to_string(),"g".to_string(),"h".to_string(),"i".to_string(),"j".to_string(),"k".to_string(),"l".to_string(),"m".to_string(),"n".to_string(),"o".to_string(),"p".to_string(),"q".to_string(),"r".to_string(),"s".to_string(),"t".to_string(),"u".to_string(),"v".to_string(),"w".to_string(),"x".to_string(),"y".to_string(),"z".to_string()];
    let chars2 = vec!["".to_string(),"0".to_string(),"1".to_string(),"2".to_string(),"3".to_string(),"4".to_string(),"5".to_string(),"6".to_string(),"7".to_string(),"8".to_string(),"9".to_string(),":".to_string(),";".to_string(),"<".to_string(),"=".to_string(),">".to_string(),"?".to_string(),"@".to_string(),"A".to_string(),"B".to_string(),"C".to_string(),"D".to_string(),"E".to_string(),"F".to_string(),"G".to_string(),"H".to_string(),"I".to_string(),"J".to_string(),"K".to_string(),"L".to_string(),"M".to_string(),"N".to_string(),"O".to_string(),"P".to_string(),"Q".to_string(),"R".to_string(),"S".to_string(),"T".to_string(),"U".to_string(),"V".to_string(),"W".to_string(),"X".to_string(),"Y".to_string(),"Z".to_string(),"a".to_string(),"b".to_string(),"c".to_string(),"d".to_string(),"e".to_string(),"f".to_string(),"g".to_string(),"h".to_string(),"i".to_string(),"j".to_string(),"k".to_string(),"l".to_string(),"m".to_string(),"n".to_string(),"o".to_string(),"p".to_string(),"q".to_string(),"r".to_string(),"s".to_string(),"t".to_string(),"u".to_string(),"v".to_string(),"w".to_string(),"x".to_string(),"y".to_string(),"z".to_string()];
    let chars3 = vec!["".to_string(),"0".to_string(),"1".to_string(),"2".to_string(),"3".to_string(),"4".to_string(),"5".to_string(),"6".to_string(),"7".to_string(),"8".to_string(),"9".to_string(),":".to_string(),";".to_string(),"<".to_string(),"=".to_string(),">".to_string(),"?".to_string(),"@".to_string(),"A".to_string(),"B".to_string(),"C".to_string(),"D".to_string(),"E".to_string(),"F".to_string(),"G".to_string(),"H".to_string(),"I".to_string(),"J".to_string(),"K".to_string(),"L".to_string(),"M".to_string(),"N".to_string(),"O".to_string(),"P".to_string(),"Q".to_string(),"R".to_string(),"S".to_string(),"T".to_string(),"U".to_string(),"V".to_string(),"W".to_string(),"X".to_string(),"Y".to_string(),"Z".to_string(),"a".to_string(),"b".to_string(),"c".to_string(),"d".to_string(),"e".to_string(),"f".to_string(),"g".to_string(),"h".to_string(),"i".to_string(),"j".to_string(),"k".to_string(),"l".to_string(),"m".to_string(),"n".to_string(),"o".to_string(),"p".to_string(),"q".to_string(),"r".to_string(),"s".to_string(),"t".to_string(),"u".to_string(),"v".to_string(),"w".to_string(),"x".to_string(),"y".to_string(),"z".to_string()];
    let chars4 = vec!["".to_string(),"0".to_string(),"1".to_string(),"2".to_string(),"3".to_string(),"4".to_string(),"5".to_string(),"6".to_string(),"7".to_string(),"8".to_string(),"9".to_string(),":".to_string(),";".to_string(),"<".to_string(),"=".to_string(),">".to_string(),"?".to_string(),"@".to_string(),"A".to_string(),"B".to_string(),"C".to_string(),"D".to_string(),"E".to_string(),"F".to_string(),"G".to_string(),"H".to_string(),"I".to_string(),"J".to_string(),"K".to_string(),"L".to_string(),"M".to_string(),"N".to_string(),"O".to_string(),"P".to_string(),"Q".to_string(),"R".to_string(),"S".to_string(),"T".to_string(),"U".to_string(),"V".to_string(),"W".to_string(),"X".to_string(),"Y".to_string(),"Z".to_string(),"a".to_string(),"b".to_string(),"c".to_string(),"d".to_string(),"e".to_string(),"f".to_string(),"g".to_string(),"h".to_string(),"i".to_string(),"j".to_string(),"k".to_string(),"l".to_string(),"m".to_string(),"n".to_string(),"o".to_string(),"p".to_string(),"q".to_string(),"r".to_string(),"s".to_string(),"t".to_string(),"u".to_string(),"v".to_string(),"w".to_string(),"x".to_string(),"y".to_string(),"z".to_string()];
    let chars5 = vec!["".to_string(),"0".to_string(),"1".to_string(),"2".to_string(),"3".to_string(),"4".to_string(),"5".to_string(),"6".to_string(),"7".to_string(),"8".to_string(),"9".to_string(),":".to_string(),";".to_string(),"<".to_string(),"=".to_string(),">".to_string(),"?".to_string(),"@".to_string(),"A".to_string(),"B".to_string(),"C".to_string(),"D".to_string(),"E".to_string(),"F".to_string(),"G".to_string(),"H".to_string(),"I".to_string(),"J".to_string(),"K".to_string(),"L".to_string(),"M".to_string(),"N".to_string(),"O".to_string(),"P".to_string(),"Q".to_string(),"R".to_string(),"S".to_string(),"T".to_string(),"U".to_string(),"V".to_string(),"W".to_string(),"X".to_string(),"Y".to_string(),"Z".to_string(),"a".to_string(),"b".to_string(),"c".to_string(),"d".to_string(),"e".to_string(),"f".to_string(),"g".to_string(),"h".to_string(),"i".to_string(),"j".to_string(),"k".to_string(),"l".to_string(),"m".to_string(),"n".to_string(),"o".to_string(),"p".to_string(),"q".to_string(),"r".to_string(),"s".to_string(),"t".to_string(),"u".to_string(),"v".to_string(),"w".to_string(),"x".to_string(),"y".to_string(),"z".to_string()];

    for i5 in &chars5{
        for i4 in &chars4{
            for i3 in &chars3{
                for i2 in &chars2{
                    for i1 in &chars1{
                        for i0 in &chars0{
                            let mut temp = format!("{}{}{}{}{}{}",i0,i1,i2,i3,i4,i5);
                            appended.push(temp);
                        }
                    }
                }
            }
        }
    }
    appended
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
