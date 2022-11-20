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
