extern crate sys_info;
extern crate termion;
mod memory;
mod terminal;
use crate::memory::{show_free_memory, show_disk, show_networks};
use crate::terminal::show_options;

fn main() {
    show_networks();
}
