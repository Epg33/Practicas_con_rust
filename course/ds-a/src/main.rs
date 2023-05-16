use std::rc::Rc;

struct List<T> {
    list: Rc<Vec<T>>
}

impl<T> List<T> {
    fn add (mut self, value: T) {
        self.list.push(value)
    }
}

fn main() {
    let mut list = Rc::new(0);

    println!("Hello, world!");
}

// fn add (value: i32, list: Rc<i32>) {
//     list.
// }