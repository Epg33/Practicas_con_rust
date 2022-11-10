use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number: i32 = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("please input your guess.");
    
    let mut guess: String = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read the line");

    println!("You guessed: {guess}");
    random();
}

fn random()->u32 {
    return 8
}
