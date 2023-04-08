fn main() {
    println!("{}", alphabet_position("The sunset sets at twelve o' clock."))
}
 
// checks if a string end with the other string that is passed
fn solution(word: &str, ending: &str) -> bool {
    if word.ends_with(ending) {
        return true;
    }
    false
}

// squares every digit of a given number a return a number that is the 
// concatenation of all theese numbers
fn square_digits(num: u64) -> u64 {
    let num_string = num.to_string();
    let mut string_for_num = String::new();
    for i in num_string.chars() {
        let i_num = i.to_digit(10).unwrap();
        string_for_num.push_str((i_num*i_num).to_string().as_str())
    }
    return string_for_num.parse::<u64>().expect("expected an unsigned number of 64 bits");
}

// takes a string and returns a string with the position in the alphabet of every char from 
// the first string
fn alphabet_position(text: &str) -> String {
    let mut result = String::new();
    for ch in text.chars() {
        if ch.is_alphabetic() && ch!=' ' {
            let position = (ch as u8 - b'a') +1;
            result.push_str(position.to_string().as_str());
            result.push_str(" ");
        }
    }
    return result;
}