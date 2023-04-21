mod memory {
  use std::io::{stdout, Write};
  use sys_info::mem_info;
  use sysinfo::{System, SystemExt, DiskExt, NetworkExt};
  use termion::clear;

  pub fn show_free_memory() {
    let mut stdout = stdout();
    loop {
        let mem = mem_info().unwrap();
        write!(stdout, "{}", clear::All).unwrap();  
        println!("{} free bytes", mem.free); 
    }
  }

  pub fn show_disk() {
    let mut sys = System::new_all();
    sys.refresh_all();
    for disk in sys.disks() {
      println!("{:?}GB of available space", disk.available_space()/1000000000);
    }
  }

  pub fn show_networks() {
    let mut sys = System::new_all();
    for (interface_name, data) in sys.networks() {
      println!("{}: {}/{} B", interface_name, data.received(), data.transmitted());
  }
  }
}

pub fn show_free_memory() {
  crate::memory::memory::show_free_memory();
}

pub fn show_disk() {
  crate::memory::memory::show_disk();
}

pub fn show_networks() {
  crate::memory::memory::show_networks();
}