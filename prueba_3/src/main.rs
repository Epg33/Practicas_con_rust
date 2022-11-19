mod incl;

fn main() {
    let mut s:String = String::from("Hello");
    s.push_str(", World");
    println!("{}", s);
    incl::printing();
    incl::scope();
}
