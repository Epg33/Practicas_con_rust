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
    
    // let path = "lines.txt";
    // let output = File::create(path);
    // let mut output = match output {
    //     Ok(file) => file,
    //     Err(err) => {panic!("Problem creating file: {:?}", err)}
    // };
    // write!(output, "Just some\nRandom Words").expect("Failed to write file");

    // let input = File::open(path).unwrap();
    // let buffer = BufReader::new(input);

    // for line in buffer.lines() {
    //     println!("{}", line.unwrap())
    // }

    // let output2 = File::create("rand.txt");
    // let mut  output2 = match output2 {
    //     Ok(file) => file,
    //     Err(err) => match err.kind() {
    //         ErrorKind::NotFound => match File::create("rand.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Cant create file: {:?}", err)
    //         },
    //         _other_error => panic!("Error opening file: {:?}", err)
    //     }
    // };
    // let rand_num = rand::thread_rng().gen_range(1..1_000_000);
    // write!(output2, "{rand_num}");

    // let mut arr_it = [1, 2, 3, 4];
    // for val in arr_it.iter() {
    //     println!("{val}");
    // }
    // let mut iter1 = arr_it.iter();
    // println!("1st: {:?}", iter1.next());

    // let can_vote = |age: i32| -> bool {
    //     age >= 18
    // };

    // println!("can vote: {:?}", can_vote(18));

    // let mut samp1 = 5;
    // let print_var = || println!("samp1 = {samp1}");
    // print_var();
    // samp1 = 10;
    // let mut change_var = || samp1 += 1;
    // change_var();
    // println!("samp1 = {samp1}");
    // samp1 = 10;
    // println!("samp1 = {samp1}");

    // fn use_function<T>(a: i32, b: i32, func: T) -> i32 
    // where T: Fn(i32, i32) -> i32 {
    //     func(a, b)
    // }
    // let sum = |a, b| a+b;
    // let prod = |a, b| a*b;
    // println!("5 + 4 = {}", use_function(5, 4, sum));
    // println!("5 * 4 = {}", use_function(5, 4, prod));

    // BOX smart pointer
    // let b_int1 = Box::new(10);
    // println!("b_int1 = {b_int1}");
    #[derive(Debug)]
    struct TreeNode<T> {
        pub left: Option<Box<TreeNode<T>>>, 
        pub right: Option<Box<TreeNode<T>>>, 
        pub key: T
    }
    impl<T> TreeNode<T> {
        pub fn new(key: T) -> Self {
            TreeNode { left: None, right: None, key }
        }
        pub fn left(mut self, node: TreeNode<T>) -> Self {
            self.left = Some(Box::new(node));
            self
        }
        pub fn right(mut self, node: TreeNode<T>) -> Self {
            self.right = Some(Box::new(node));
            self
        }
    }

    let node1 = TreeNode::new(1).left(TreeNode::new(2)).right(TreeNode::new(3));
    println!("{:?}", node1);

}