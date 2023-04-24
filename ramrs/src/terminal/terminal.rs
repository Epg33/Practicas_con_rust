pub mod terminal {
  use terminal_menu::{menu, run, label, button, mut_menu};
  use crate::cpu::cpu::cpu::show_cpu_usage;
  pub fn show_principal_menu() {
    let menu = menu(vec![
      button("Disk"),
      button("Ram"),
      button("Cpu")
    ]);
    run(&menu);
    let mutmen = mut_menu(&menu);
    match mutmen.selected_item_name() {
        "Disk" => show_disk_options(),
        "Ram" => println!("Show Ram options"),
        "Cpu" => show_cpu_options(),
        &_ => println!("Select a valid option")
    }
  }

  pub fn show_disk_options() {
    let menu = menu(vec![
      button("Disk usage"),
      button("Dirs"),
      button("Go Back")
    ]);
    run(&menu);
    let mutmen = mut_menu(&menu);
    match mutmen.selected_item_name() {
        "Disk usage" => println!("Show disk usage"),
        "Dirs" => println!("Show Directories and options"),
        "Go Back" => show_principal_menu(),
        &_ => println!("Select a valid option")
    }
  }

  pub fn show_cpu_options () {
    let menu = menu(vec![
      button("show cpus usage")
    ]);
    run(&menu);
    let mutmen = mut_menu(&menu);
    match mutmen.selected_item_name() {
      "show cpus usage" => show_cpu_usage(),
      &_ => {println!("Select a valid option"); show_cpu_options()}
    }
  }
}
