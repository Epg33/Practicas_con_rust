#![allow(unused)]

use core::num;
use std::{io, vec};
use rand::Rng;
use std::io::{Write, BufReader, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use std::collections::HashMap;

fn main() {
    println!("what is your name?");
    let mut name = String::new();
    let greeting = "Nice to meet you";
    io::stdin().read_line(&mut name)
        .expect("didn´t recieve input");
    println!("Hello, {}! {}", name.trim_end(), greeting);

    const ONE_MIL:u32 = 1_000_000;
    const PI: f32 = 3.141592;
    let age:&str = "47";
    let mut age: u32 = age.trim().parse().expect("age wasn´t asigned a number");
    age +=1;
    println!("I'm {} and I want ${}", age, ONE_MIL);

    println!("Max u32: {}", u32::MAX);
    println!("Max u64: {}", u64::MAX);
    println!("Max usize: {}", usize::MAX);
    println!("Max u128: {}", u128::MAX);
    println!("Max f32: {}", f32::MAX);
    println!("Max f64: {}", f64::MAX);

    let num1:f32 = 1.111111111111111;
    println!("f32: {}", num1+0.111111111111111);
    let num2:f64 = 1.111111111111111;
    println!("f64: {}", num2+0.111111111111111);

    let random_num = rand::thread_rng().gen_range(1..101);
    println!("Random: {random_num}");

    let age:i32 = 8;
    if(age>=1) && (age<=18){
        println!("Important Birthday");
    } else if (age==21) || (age==50) {
        println!("yes");
    } else if age>=65 {
        println!("no")
    } else {
        println!("maybe")
    }

    let mut my_age = 47;
    let can_vote = if my_age>=18 {true} else {false};
    println!("Can vote?: {can_vote}");

    let age2 = 8;
    match age2 {
        1..=18 => println!("Important Birthday"),
        21 | 50 => println!("yes"),
        65..=i32::MAX => println!("NO"),
        _ => println!("Maybe")
    }

    let my_age = 18;
    let voting_age = 18;
    match my_age.cmp(&voting_age) {
        Ordering::Less => println!("Cant Vote"),
        Ordering::Greater => println!("Can Vote"),
        Ordering::Equal => println!("Now You Cant Vote"),
    }

    let arr1 = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    println!("1st: {}", arr1[0]);
    println!("Length: {}", arr1.len());
    let arr2 = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut loop_idx = 0;
    loop {
        if arr2[loop_idx] % 2 == 0 {
            loop_idx += 1;
            continue;
        }
        if arr2[loop_idx] == 9 {
            break;;
        }
        loop_idx += 1;
        println!("Val: {}", arr2[loop_idx])
    }
    while loop_idx < arr2.len() {
        println!("Arr: {}", arr2[loop_idx]);
    }

    for val in arr2.iter() {
        println!("Arr: {}", val);
    }

    let my_tuple: (u8, String, f64) = (47, "Alex".to_string(), 50.000);
    println!("Name: {}", my_tuple.1);
    let (v1, v2, v3) = my_tuple;
    println!("Age: {}", v1);

    let mut st1 = String::new();
    st1.push('A');
    st1.push_str(" word");
    for word in st1.split_whitespace() {
        println!("{word}");
    }
    let st2 = st1.replace("A", "another");
    println!("{st2}");

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
    let st6 = String::from("Just Some");
    let st7 = String::from(" words");
    let st8 = st6 + & st7;
    for char in st8.bytes() {
        println!("{char}");
    }

    let int_u8:u8 = 5;
    let int2_u8:u8 = 4;
    let int_u32:u32 = (int_u8 as u32) + (int2_u8 as u32);

    enum Day {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday
    }

    impl Day {
        fn is_weekend(&self) -> bool {
            match self {
                Day::Saturday | Day::Sunday => true,
                _ => false 
            }
        }
    }

    let today:Day = Day::Monday;
    match today {
        Day::Monday => println!("Fuck monday"),
        Day::Tuesday => println!("Fuck Tuesday"),
        Day::Wednesday => println!("Fuck Wednesday"),
        Day::Thursday => println!("Fuck Thursday"),
        Day::Friday => println!("Fuck Friday"),
        Day::Saturday => println!("Fuck Saturday"),
        Day::Sunday => println!("Fuck Sunday"),
    }
    println!("Is today the weekend?: {}", today.is_weekend());

    let vec1:Vec<i32> = Vec::new();
    let mut vec2 = vec![1, 2, 3, 4];
    vec2.push(5);
    println!("1st {}", vec2[0]);
    let second: &i32 = &vec2[1];
    match vec2.get(1) {
       Some(second) => println!("2nd: {}", second),
       None => println!("No second value") 
    };
    for i in &mut vec2 {
        *i *= 2;
    }
    for i in &vec2 {
        println!("{i}")
    }
    println!("Vector length {}", vec2.len());
    println!("Pop: {:?}", vec2.pop());
    println!("{}", get_some(5, 10));
    let (val1, val2) = get2(10);
    println!("Nums: {val1} {val2}");
    let num_list = vec![1, 2, 3, 4, 5];
    println!("Sum of list = {}", sum_list(&num_list));

    println!("5 + 4 = {}", get_sum_gen(5, 4));
    let str1 = String::from("Word");
    let str2 = str1.clone();
    print_str(str1);
    let mut str3 = String::from("Alex");
    change_str(&mut str3);
    
    let mut heroes = HashMap::new();
    heroes.insert("Superman", "Clark Kent");
    heroes.insert("Batman", "Bruce Wayne");
    heroes.insert("Flash", "Barry Allen");
    for (k, v) in heroes.iter() {
        println!("{k} = {v}");
    }

    println!("{}", heroes.len());
    if heroes.contains_key(&"Batman") {
        let the_batman = heroes.get(&"Batman");
        match the_batman {
            Some(x) => println!("I am Batman"),
            None => println!("Who?")
        }
    }

    struct Customer {
        name: String,
        addres: String,
        balance: f32
    }
    let mut bob = Customer{
        name: String::from("Bob"),
        addres: String::from("555 main st"),
        balance: 234.50
    };
    bob.addres = String::from("505 Main st");

    struct Rectangle<T, U> {
        length: T,
        height: U
    };
    let rec = Rectangle{length: 4, height:10.5};
    trait Shape {
        fn new(length: f32, width: f32) -> Self;
        fn area(&self) -> f32;
    };
    struct Rectangle2 {
        length: f32,
        width: f32
    };
    struct Circle {
        length: f32,
        width: f32
    };
    impl Shape for Rectangle2 {
        fn new(length: f32, width: f32) -> Rectangle2 {
            return Rectangle2{length, width};
        }
        fn area(&self) -> f32 {
            return self.length * self.width;
        }
    }
    impl Shape for Circle {
        fn new(length: f32, width: f32) -> Circle {
            return Circle{length, width};
        }
        fn area(&self) -> f32 {
            return (self.length / 2.0 ).powf(2.0) * PI;
        }
    }
    let rec2: Rectangle2 = Shape::new(10.0, 10.0);
    let circ: Circle = Shape::new(10.0, 10.0);
    println!("Rec Area: {}", rec2.area());
    println!("Circ Area: {}", circ.area());
}

fn print_str(x:String) {
    println!("A string {x}");
}

fn print_return_str(x:String) -> String {
    println!("A string {x}");
    x
}

fn change_str(name:&mut String) {
    name.push_str(" Is Happy");
    println!("Mesagge: {name}");
}

use std::ops::Add;

fn get_sum_gen<T:Add<Output = T>>(x: T, y: T) -> T {
    return x + y;
}

fn sum_list (list: &[i32]) -> i32 {
    let mut sum = 0;
    for &val in list.iter() {
        sum += &val;
    }
    sum
}

fn get2 (x:i32) -> (i32, i32) {
    return (x+1, x+2);
}
fn get_some(x:i32, y:i32) -> i32 {
    return x + y;
}