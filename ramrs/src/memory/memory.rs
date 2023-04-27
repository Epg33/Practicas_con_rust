pub mod memory {
  use sysinfo::{System, SystemExt, DiskExt, NetworkExt, ProcessExt};

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
}