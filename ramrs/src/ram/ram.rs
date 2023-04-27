pub mod ram {
  use sys_info::mem_info;
  use sysinfo::{System, SystemExt, Pid, Process};
  use terminal_menu::{menu, run, button, mut_menu};
  pub fn show_free_memory() {
    loop {
        print!("\x1B[2J\x1B[1;1H");
        let mem = mem_info().unwrap();
        println!("{} free bytes", mem.free); 
    }
  }

  pub fn show_processes() {
    let sys = System::new_all();
    print!("\x1B[2J\x1B[1;1H");
    let menu_buttons: Vec<&Process> = sys.processes().iter()
      .map(|process| process.1).collect();
    // let menu = menu(menu_buttons.iter().map(||button));
        // process.kill();

      // println!("[{}] {} {:?}", pid, process.name(), process.disk_usage());
    
  }
}

