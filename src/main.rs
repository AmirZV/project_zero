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

// fn main() {
//     const ONE_MIL: u32 = 1_000_000;
//     const PI: f32 = 3.14159;
//     let age = "25";
//     let mut age: u32 = age.trim().parse()
//         .expect("Age wasn't assigned a number");
//     age += 1;
//     println!("I'm {} and I want ${}", age, ONE_MIL);

// }

// fn main() {
//     // Unsiged integer : u8, u16, u32, u64, u128, usize
//     // Signed integer : i8, i16, i32, i64, i128, isize
//     // Float : f32, f64

//     println!("Max u32 : {}", u32::MAX);
//     println!("Max u64 : {}", u64::MAX);
//     println!("Max usize : {}", usize::MAX);
//     println!("Max u128 : {}", u128::MAX);

//     println!("Max f32: {}", f32::MAX);
//     println!("Max f64 : {}", f64::MAX);

//     let is_true = true;
//     //let _is_true = true; Put an underscore and Rust compilor ignore it if it is unused

//     let my_grade = 'A';
// }

fn main() {
    let num_1: f32 = 1.111111111111111;
    println!("f32 : {}", num_1 + 0.111111111111111); // 6 digits accuracy
    let num_2: f64 = 1.111111111111111;
    println!("f64 : {}", num_2 + 0.111111111111111); // 14 digits accuracy

    let num_3: u32 = 6;
    let num_4: u32 = 4;
    println!("6 + 4 = {}", num_3 + num_4);
    println!("6 - 4 = {}", num_3 - num_4);
    println!("6 * 4 = {}", num_3 * num_4);
    println!("6 / 4 = {}", num_3 / num_4);
    println!("6 % 4 = {}", num_3 % num_4);
}