extern crate sys_info;
extern crate termion;
mod memory;
mod terminal;
mod cpu;
use crate::memory::memory::memory::show_processes;
use crate::terminal::terminal::terminal::show_principal_menu;
// use crate::terminal::show_options;

fn main() {
    show_principal_menu();
}
