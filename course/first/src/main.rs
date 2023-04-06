#![allow(unused)]

use core::num;
use std::fmt::write;
use std::{io, vec};
use rand::Rng;
use std::io::{Write, BufReader, ErrorKind, BufRead};
use std::fs::File;
use std::cmp::Ordering;
use std::collections::HashMap;

//mod restaurant;
//use crate::restaurant::order_food;

fn main() {
    //order_food();
    
    let path = "lines.txt";
    let output = File::create(path);
    let mut output = match output {
        Ok(file) => file,
        Err(err) => {panic!("Problem creating file: {:?}", err)}
    };
    write!(output, "Just some\nRandom Words").expect("Failed to write file");

    let input = File::open(path).unwrap();
    let buffer = BufReader::new(input);

    for line in buffer.lines() {
        println!("{}", line.unwrap())
    }

    let output2 = File::create("rand.txt");
    let mut  output2 = match output2 {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("rand.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Cant create file: {:?}", err)
            },
            _other_error => panic!("Error opening file: {:?}", err)
        }
    };
    let rand_num = rand::thread_rng().gen_range(1..1_000_000);
    write!(output2, "{rand_num}");
}