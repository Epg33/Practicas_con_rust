use rand::Rng;

pub fn printing() {
    let num: u32 = rand::thread_rng().gen_range(1..=10);
    println!("{num}");
}