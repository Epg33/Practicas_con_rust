use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=10);

    let mut tries:u32 = 0;

    loop {
        let tries_left:u32 = 5-tries;
        if tries_left==0{
            println!("you have no more tries left");
            break;
        };
        println!("You have {tries_left} tries left");
        println!("please input your guess.");

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You Won!!");
                break;
            }
        }

        tries+=1;
    }
}
