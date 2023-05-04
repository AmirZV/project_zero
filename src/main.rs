#![allow(unused)]

use std::{io, string};
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

// fn main() {
//     println!("Hello World!");
// }

fn main () {
    println!("Enter your name: ");
    let mut name = String::new();
    let greeting = "Nice to meet you";
    io::stdin().read_line(&mut name)
        .expect("Dind't Receive Input!");
    println!("Hello {}!, {}", name.trim_end(), greeting);
}