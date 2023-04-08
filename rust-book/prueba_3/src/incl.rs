use rand::Rng;

pub fn printing() {
    let num: u32 = rand::thread_rng().gen_range(1..=10);
    println!("{num}");
}

pub fn scope() {
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{s2}, World!");

    another_scope();
}

pub fn another_scope() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");
}

pub fn lengths() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The length of {s1} is {len}");
}

pub fn calculate_length(s:&String) -> usize {
    return s.len();
}

pub fn to_change() {
    let mut cons = String::from("Constant");
    change(&mut cons)
    // not_changing(&cons);
}

pub fn change(the_str: &mut String){
    the_str.push_str(", change?");
}

// pub fn not_changing(the_str: &String){
//     the_str.push_str(", change?");
// }