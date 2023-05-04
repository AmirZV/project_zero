#![allow(unused)]

use std::{io, string};
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

// fn main() {
//     println!("Hello World!");
// }

// fn main () {
//     println!("Enter your name: ");
//     let mut name = String::new();
//     let greeting = "Nice to meet you";
//     io::stdin().read_line(&mut name)
//         .expect("Dind't Receive Input!");
//     println!("Hello {}!, {}", name.trim_end(), greeting);
// }

fn main() {
    const ONE_MIL: u32 = 1_000_000;
    const PI: f32 = 3.14159;
    let age = "25";
    let mut age: u32 = age.trim().parse()
        .expect("Age wasn't assigned a number");
    age += 1;
    println!("I'm {} and I want ${}", age, ONE_MIL);

}