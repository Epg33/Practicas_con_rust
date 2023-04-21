extern crate sys_info;
extern crate termion;
mod memory;
mod terminal;
use crate::memory::memory::memory::show_processes;
// use crate::terminal::show_options;

fn main() {
    show_processes();
}
