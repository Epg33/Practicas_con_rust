pub mod terminal {
  use terminal_menu::{menu, run, label, button, mut_menu};
  pub fn show_principal_menu() {
    let menu = menu(vec![
      button("Disk"),
      button("Ram")
    ]);
    run(&menu);
    let mutmen = mut_menu(&menu);
    match mutmen.selected_item_name() {
        "Disk" => show_disk_options(),
        "Ram" => println!("Show Ram options"),
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
}
