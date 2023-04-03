#![allow(unused)]

use core::num;
use std::io;
use rand::Rng;
use std::io::{Write, BufReader, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    // println!("what is your name?");
    // let mut name = String::new();
    // let greeting = "Nice to meet you";
    // io::stdin().read_line(&mut name)
    //     .expect("didn´t recieve input");
    // println!("Hello, {}! {}", name.trim_end(), greeting);

    // const ONE_MIL:u32 = 1_000_000;
    // const PI: f32 = 3.141592;
    // let age:&str = "47";
    // let mut age: u32 = age.trim().parse().expect("age wasn´t asigned a number");
    // age +=1;
    // println!("I'm {} and I want ${}", age, ONE_MIL)

    // println!("Max u32: {}", u32::MAX);
    // println!("Max u64: {}", u64::MAX);
    // println!("Max usize: {}", usize::MAX);
    // println!("Max u128: {}", u128::MAX);
    // println!("Max f32: {}", f32::MAX);
    // println!("Max f64: {}", f64::MAX);

    // let num1:f32 = 1.111111111111111;
    // println!("f32: {}", num1+0.111111111111111);
    // let num2:f64 = 1.111111111111111;
    // println!("f64: {}", num2+0.111111111111111)

    // let random_num = rand::thread_rng().gen_range(1..101);
    // println!("Random: {random_num}");

    // let age:i32 = 8;
    // if(age>=1) && (age<=18){
    //     println!("Important Birthday");
    // } else if (age==21) || (age==50) {
    //     println!("yes");
    // } else if age>=65 {
    //     println!("no")
    // } else {
    //     println!("maybe")
    // }

    // let mut my_age = 47;
    // let can_vote = if my_age>=18 {true} else {false};
    // println!("Can vote?: {can_vote}")

    // let age2 = 8;
    // match age2 {
    //     1..=18 => println!("Important Birthday"),
    //     21 | 50 => println!("yes"),
    //     65..=i32::MAX => println!("NO"),
    //     _ => println!("Maybe")
    // }

    // let my_age = 18;
    // let voting_age = 18;
    // match my_age.cmp(&voting_age) {
    //     Ordering::Less => println!("Cant Vote"),
    //     Ordering::Greater => println!("Can Vote"),
    //     Ordering::Equal => println!("Now You Cant Vote"),
    // }

    // let arr1 = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    // println!("1st: {}", arr1[0]);
    // println!("Length: {}", arr1.len());
    // let arr2 = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    // let mut loop_idx = 0;
    // loop {
    //     if arr2[loop_idx] % 2 == 0 {
    //         loop_idx += 1;
    //         continue;
    //     }
    //     if arr2[loop_idx] == 9 {
    //         break;;
    //     }
    //     loop_idx += 1;
    //     println!("Val: {}", arr2[loop_idx])
    // }
    // while loop_idx < arr2.len() {
    //     println!("Arr: {}", arr2[loop_idx]);
    // }

    // for val in arr2.iter() {
    //     println!("Arr: {}", val);
    // }

    // let my_tuple: (u8, String, f64) = (47, "Alex".to_string(), 50.000);
    // println!("Name: {}", my_tuple.1);
    // let (v1, v2, v3) = my_tuple;
    // println!("Age: {}", v1)

    // let mut st1 = String::new();
    // st1.push('A');
    // st1.push_str(" word");
    // for word in st1.split_whitespace() {
    //     println!("{word}");
    // }
    // let st2 = st1.replace("A", "another");
    // println!("{st2}");

    let st3 = String::from("x r t b h k k a m c");
    let mut v1:Vec<char> = st3.chars().collect();
    v1.sort();
    v1.dedup();
    for char in v1 {
        println!("{char}");
    }

    let st4: &str = "Random string";
    let mut st5: String = st4.to_string();
    println!("{st5}");
    let byte_arr1 = st5.as_bytes();
    let st6 = &st5[0..6];
    println!("{st6}");
    println!("String length: {}", st6.len());
    st5.clear();
}
