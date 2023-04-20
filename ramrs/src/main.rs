extern crate sys_info;
extern crate termion;
mod memory;
use crate::memory::show_free_memory;

fn main() {
    show_free_memory();
}
