pub mod memory {
  use sys_info::mem_info;
  use sysinfo::{System, SystemExt, DiskExt, NetworkExt, ProcessExt};

  pub fn show_free_memory() {
    loop {
        print!("\x1B[2J\x1B[1;1H");
        let mem = mem_info().unwrap();
        println!("{} free bytes", mem.free); 
    }
  }

  pub fn show_disk() {
    let mut sys = System::new_all();
    sys.refresh_all();
    print!("\x1B[2J\x1B[1;1H");
    for disk in sys.disks() {
      println!("-------------------------------------");
      println!("Disk: {:?}", disk.name());
      println!("Space: {:?}GB available, {:?}GB total", disk.available_space()/1000000000, disk.total_space()/1000000000);
      println!("-------------------------------------");
    }
  }

  pub fn show_networks() {
    let sys = System::new_all();
    print!("\x1B[2J\x1B[1;1H");
    for (interface_name, data) in sys.networks() {
      println!("{}: {}/{} B", interface_name, data.received(), data.transmitted());
    }
  }

  pub fn show_processes() {
    let sys = System::new_all();
    print!("\x1B[2J\x1B[1;1H");
    for (pid, process) in sys.processes() {
      if process.name()=="code" {
        println!("{}", process.name());
        // process.kill();
      }
      // println!("[{}] {} {:?}", pid, process.name(), process.disk_usage());
    }
  }
}