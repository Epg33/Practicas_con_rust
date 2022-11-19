use rand::Rng;

pub fn printing() {
    let num: u32 = rand::thread_rng().gen_range(1..=10);
    println!("{num}");
}

pub fn scope() {
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{s1}, World!");
}