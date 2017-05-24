# Rust the Ripper

## Description
John the Ripper is a staple of any good hackers toolset. It allows you to tackle quickly and easily attack different password hashes from a variety of angles. We have attempted to create a cli with similar functionality using the Rust programming language. The implementation takes advantage of Rust ability to safely use concurrency quickly try to crack as many passwords as possible.

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
$  ./ripper --hashes hashes.txt --wordlist wordlist.txt -t 20
```
