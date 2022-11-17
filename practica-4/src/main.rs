fn main() {
    let x:i32 = 5;

    let x: i32 = x +1;

    {
        let x:i32 = x *2;
        println!("The value of x in this scope is {x}")
    }

    println!("The value of x in this scope is {x}");

    let spaces = "    ";
    let spaces = spaces.len();
    println!("{spaces}");

    const TUP: (i32, f64, u8) = (500, 6.4, 1);
    println!("{:#?}", TUP.0);

    let nums: [i32; 10] = [1, 2, 3, 4, 5, 6,7, 8, 9, 10];
    for i in nums {
        println!("{i}");
    }
}
