mod terminal {
  use terminal_menu::{menu, run, label, button, mut_menu};
  pub fn show_options() {
    let menu = menu(vec![
      button("Show free memory")
    ]);
    run(&menu);
    let mutmen = mut_menu(&menu);
    println!("{}", mutmen.selected_item_name())
  }
}

pub fn show_options() {
  crate::terminal::terminal::show_options();
}