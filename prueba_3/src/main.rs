mod incl;

fn main() {
    let mut s:String = String::from("Hello");
    s.push_str(", World");
    println!("{}", s);
    incl::printing();
    incl::scope();

    let stri = String::from("hey");
    takes_ownership(stri);
    // println!("{stri}");

    let x = 5;
    makes_copy(x);
    // println!("{x}");

}

fn takes_ownership(sm_str:String) {
    println!("{sm_str}");
}

fn makes_copy (sm_int:i32) {
    println!("{sm_int}");
}
