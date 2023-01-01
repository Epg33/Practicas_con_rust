use clap::Parser;

//Search for a pattern in a file and display the lines that contain it
#[derive(Parser)]
#[derive(Debug)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    // let content = std::fs::read_to_string(&args.path).expect("could not read file");

    let result = std::fs::read_to_string("test.txt");
    let content = match result {
        Ok(content) => {content},
        Err(error) => { return Err(error.into()); }
    };

    println!("file content: {content}");
    Ok(())

    // for line in content.lines() {
    //     if line.contains(&args.pattern){
    //         println!("{line}");
    //     }
    // }
}
