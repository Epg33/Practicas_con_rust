mod memory {
  use std::io::{stdout, Write};
  use sys_info::mem_info;
  use termion::clear;

  pub fn show_free_memory() {
    let mut stdout = stdout();
    loop {
        let mem = mem_info().unwrap();
        write!(stdout, "{}", clear::All).unwrap();  
        println!("{} free bytes", mem.free); 
    }
  }
}

pub fn show_free_memory() {
  crate::memory::memory::show_free_memory();
}