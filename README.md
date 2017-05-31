# Rust the Ripper

## Description
John the Ripper is a staple of any good hacker's tool set. It allows you to  quickly and easily tackle different password hashes from a variety of angles. We have attempted to create a CLI with functionality similar to that of John the Ripper using the Rust programming language. The implementation takes advantage of Rust's ability to safely use concurrency to quickly attempt to crack some pa$$w0rds.

## [TODOS](https://docs.google.com/a/u.northwestern.edu/document/d/1GiCuUi17eBSVrJ-CwFJi6_JRyVOQCF5WnB72vBVG3Us/edit?usp=sharing)


## Install
* Install rust
```
$ curl https://sh.rustup.rs -sSf | sh
```
* Clone this repo

## Getting Started
* Run the ripper binary specifying a hashfile and a wordlist file
```
$ ./ripper --hashes hashes.txt --wordlist wordlist.txt
```
* Optionally, you can specify the number of threads that you would like to run on (the default is 10)
```
$  ./ripper --hashes hashes.txt --wordlist wordlist.txt --threads 20
```
## Rules and Mangling
You can modify the [mangle function](https://github.com/sgp715/rust_the_ripper/blob/master/src/main.rs#L21) in main.rs to include any mangling rules you need implemented. As an example and by default it will run brute force generating all possible strings of length at most 6 if a blank line is present in the wordlist file. Additionally it will run the append_numbers mangle to any other words in the wordlist adding numbers 0-9999 to the end of the words.
