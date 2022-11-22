use std::io;
mod operations;

fn main() {
    println!("Please input the numbers you want");
    println!("first");
    let mut inp_num1:i32 = 0;
    let mut str1:String = String::new();
    io::stdin()
        .read_line(&mut str1)
        .expect("insert a valid number");
    inp_num1= str1.parse::<i32>().unwrap();

    println!("second");
    let mut inp_num2:i32 = 0;
    let mut str1:String =String::new();
    io::stdin()
        .read_line(&mut str1)
        .expect("insert a valid number");
    inp_num2 = str1.parse::<i32>().unwrap();

    println!("please input the sign of the operation you want: +, -, /, *");
    let mut desicion:String = String::new();
    io::stdin()
        .read_line(&mut desicion)
        .expect("insert a valid operation");
    
    let operation:i32=options(&desicion, inp_num1, inp_num2);
    println!("{operation}")
}

fn options(opt: &str, num1:i32, num2:i32) -> i32 {
    let mut result: i32 = 0;
    match opt {
        "+" => result = operations::adding(num1, num2),
        "-" => result = operations::substrackting(num1, num2),
        "*" => result = operations::multipliying(num1, num2),
        "/" => result = operations::dividing(num1, num2),
        &_ => println!("insert a valid operation"),
    };
    return result;
}
