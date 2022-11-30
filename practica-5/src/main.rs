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