mod slices;

fn main() {
    let x:i32 = 5;

    let x: i32 = x +1;

    {
        let x:i32 = x *2;
        println!("The value of x in this scope is {x}")
    }

    println!("The value of x in this scope is {x}");

    let spaces:&str = "    ";
    let spaces:usize = spaces.len();
    println!("{spaces}");

    const TUP: (i32, f64, u8) = (500, 6.4, 1);
    println!("{:#?}", TUP.0);

    let nums: [i32; 10] = [1, 2, 3, 4, 5, 6,7, 8, 9, 10];
    for i in nums {
        println!("{i}");
    }

    let bind:i32 = {
        let y:i32 = 5;
        y*y
    };
    println!("{bind}");

    let add:u32 = adding(10, 10);

    println!("{add}");

    looping();

    let mut s:String = String::from("Hello World");
    let word:&str= slices::first_word(&s);
    s.clear();
}

fn adding(num1:u32, num2:u32)->u32 {
    let result:u32 = num1+ num2;
    return result;
}

fn looping() {
    for number in (1..50).rev(){
        println!("{number}");
    }
    println!("ya");
}
