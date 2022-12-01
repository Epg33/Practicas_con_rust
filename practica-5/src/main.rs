use std::io;

#[derive(Debug)]
struct Rectangle{
    width:i32,
    heigth:i32
}

struct User {
    active: bool,
    user_name: String,
    email: String,
    id: u64
}
struct Rgb(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let user1 = User{
        active: true,
        user_name: String::from("YeahItsMe"),
        email: String::from("a@gmail.com"),
        id: 123 
    };
    let user2 = User {
        email: String::from("b@gmail.com"),
        ..user1
    };

    println!("please input the width and the heigth of the rectangle");
    println!("width: ");
    let mut rec:String =String::new().to_string();

    io::stdin()
        .read_line(&mut rec)
        .expect("insert a valid number");

    let width:i32 = rec.trim().parse::<i32>().unwrap();

    println!("heigth: ");

    let mut rec:String =String::new().to_string();

    io::stdin()
        .read_line(&mut rec)
        .expect("insert a valid number");

    let heigth:i32 = rec.trim().parse::<i32>().unwrap();
    let rectagle:Rectangle= Rectangle { width, heigth };
    println!("the rectangle is {:?}, and it's area is {}", rectagle, rectagle.area_rectangle());
    let rectangle2:Rectangle = Rectangle { width: 50, heigth: 50 };
    println!("can recatngle 1 fit a 50 x 50 rectangle: {}", rectagle.can_hold(&rectangle2))
}

fn build_user(email:String, user_name:String)->User{
    let user = User{
        email,
        user_name,
        active: false,
        id: 1234
    };
    return  user;
}
impl Rectangle {
    fn area_rectangle(&self)->i32{
        self.heigth*self.width
    }    

    fn can_hold(&self, other: &Rectangle)->bool {
        self.width>other.width && self.heigth > other.heigth
    }
}


