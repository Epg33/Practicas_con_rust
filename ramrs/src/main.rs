extern crate sys_info;
extern crate termion;
mod memory;
mod terminal;
mod cpu;
use crate::terminal::terminal::terminal::show_principal_menu;

fn main() {
    show_principal_menu();
}
