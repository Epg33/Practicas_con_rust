use std::result;

mod operations;

fn main() {
    println!("Hello, world!");
    let operation=options("+");
    println!("{operation}")
}

fn options(opt: &str) -> i32 {
    let mut result: i32 = 0;
    match opt {
        "+" => result = operations::adding(10, 15),
        "-" => result = operations::substrackting(15, 20),
        "*" => result = operations::multipliying(10, 10),
        "/" => result = operations::dividing(10, 10),
        &_ => println!("insert a valid operation"),
    };
    return result;
}
