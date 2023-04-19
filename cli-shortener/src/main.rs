use clap::Parser;

#[derive(Parser)]
struct CliArgs {
    params: String,
    command: String
}

fn main() {
    let args = CliArgs::parse();
    match args.params {
        "new" => println!("New command"),
        "insert" => println!("Insert command"),
        "erase" => println!("Erase command"),
        "update" => println!("Update command"),
    }
}
