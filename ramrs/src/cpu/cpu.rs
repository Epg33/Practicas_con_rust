pub mod cpu {
  use terminal_menu::{menu, run, button, mut_menu};
  use sysinfo::{System, SystemExt, CpuExt, UserExt};
  use crate::terminal::terminal::terminal::show_principal_menu;
  pub fn show_cpu_usage() {
    let mut sys = System::new_all();
    loop {
      sys.refresh_cpu();
      print!("\x1B[2J\x1B[1;1H"); 
      for cpu in sys.cpus() {
        println!("----------------{}----------------", cpu.name());
        println!("INFO:");
        println!("Is using: {}%",cpu.cpu_usage().round());
        println!("It has a {}MHz frequency", cpu.frequency());
      }
      std::thread::sleep(std::time::Duration::from_millis(500))
    }
  }
  pub fn show_system_info() {
    let sys = System::new_all();
    print!("\x1B[2J\x1B[1;1H");
    println!("----------------------System Info----------------------");
    println!("System Name: {}", sys.name().unwrap());
    println!("System Kernel Version: {:?}", sys.kernel_version());
    println!("System OS Version: {:?}", sys.os_version());
    println!("System Host Name: {:?}", sys.host_name());
    for component in sys.components() {
      println!("{:?}", component);
    }
    let menu = menu(vec![
      button("Go Back")
    ]);
    run(&menu);
    let mutmen = mut_menu(&menu);
    match mutmen.selected_item_name() {
        "Go Back" => show_principal_menu(),
        &_ => println!("Select a valid option")
    }
  }
  
}