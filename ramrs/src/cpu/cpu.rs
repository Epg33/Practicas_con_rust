pub mod cpu {
  use std::io::{stdout, Write};
  use termion::clear;
  use sysinfo::{System, SystemExt, DiskExt, NetworkExt, ProcessExt, CpuExt};
  pub fn show_cpu_usage() {
    let mut sys = System::new_all();
    loop {
      sys.refresh_cpu();
      write!(stdout(), "{}", clear::All).unwrap(); 
      for cpu in sys.cpus() {
        println!("{} is using: {}%", cpu.name(), cpu.cpu_usage().round())
      }
      std::thread::sleep(std::time::Duration::from_millis(500))
    }
  }
}