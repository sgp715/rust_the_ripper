# Rust the Ripper

## Description
John the Ripper is a staple of any good hacker's tool set. It allows you to  quickly and easily tackle different password hashes from a variety of angles. We have attempted to create a CLI with functionality similar to that of John the Ripper using the Rust programming language. The implementation takes advantage of Rust's ability to safely use concurrency to quickly attempt to crack some pa$$w0rds.

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
