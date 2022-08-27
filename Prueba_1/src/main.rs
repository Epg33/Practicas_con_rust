fn main(){
    let tuple:(i32, &str) = (1, "que onda");
    let array:[i32; 10]=[1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let a_arr = &array[..];
    println!("an array{:?} and a tuple{:?}", a_arr, tuple);
    sum(12, 15);
    println!("{}", sum(12, 15));
}

fn sum(num:i32, num2:i32)->i32{
    let result:i32 = num + num2;
    return result;
}
