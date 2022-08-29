fn main() {
    let letter: char = 'x';
    let string: &str = "biv";
    print!("a char {}, a string {} ", letter, string);
    println!("{}", suma(15, 128));
    data();
    try_enums();
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
        lastname: String,
        age: u8,
        experience: u8,
        remote: bool,
    }
    let yo = Person {
        name: String::from("Ethiem"),
        lastname: String::from("Guerrero"),
        age: 18,
        experience: 7,
        remote: false,
    };
    print!("my name is {} {}, im {} years old, with {} months of programing, am i a remote student? {}", yo.name, yo.lastname, yo.age, yo.experience, yo.remote);
}

fn try_enums () {
    #[derive(Debug)]
    struct KeyPress (String, char);

    #[derive(Debug)]
    struct MouseClick {x: i64, y: i64}

    let click: MouseClick = MouseClick { x: 10, y: 20 };
    let keys= KeyPress(String::from("Control+"), 'N');
    
    #[derive(Debug)]
    enum WebEvent {
        WELoad(bool),
        WEClick(MouseClick),
        WEKeys(KeyPress)
    }

    let we_load: WebEvent = WebEvent::WELoad(true);
    let we_click: WebEvent = WebEvent::WEClick(click);
    let we_keys: WebEvent = WebEvent::WEKeys(keys);
    println!("\nWebEvent enum structure: \n\n {:#?} \n\n {:#?} \n\n {:#?}", we_load, we_click, we_keys)
}
