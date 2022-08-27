fn main() {
    let letter: char = 'x';
    let string: &str = "biv";
    print!("a char {}, a string {} ", letter, string);
    println!("{}", suma(15, 128));
    data();
}

fn suma(num1: i32, num2: i32) -> i32 {
    let result: i32 = num1 + num2;
    if result > 10 {
        return result;
    } else {
        return 0;
    }
}

fn data() {
    let tupla: (i32, &str, bool, i32) = (2, "a tuple", true, 6);
    print!("{}", tupla.0);
    struct Person {
        name: String,
        age: u8,
        level: u8,
        remote: bool,
    }
    let yo = Person {
        name: String::from("Ethiem"),
        age: 18,
        level: 3,
        remote: false,
    };
    print!("my name is {}, im {} years old, im in level {} of programing, am i a remote student? {}", yo.name, yo.age, yo.level, yo.remote);
}
