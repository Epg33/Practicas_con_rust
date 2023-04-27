extern crate sys_info;
extern crate termion;
mod terminal;
mod cpu;
mod ram; 
mod memory;
use crate::terminal::{terminal::terminal::show_principal_menu, tui::tui::terminal};

fn main() {
    // show_principal_menu();
    terminal().unwrap();
}
