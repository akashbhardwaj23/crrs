use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern : String,
    path : std::path::PathBuf
}

fn main() -> Result<(), Box<dyn std::error::Error>> { 
    // let pattern = std::env::args().nth(1).expect("no pattern given");
    // let path = std::env::args().nth(2).expect("No path Given");

    // let cli = Cli{
    //     pattern,
    //     path : std::path::PathBuf::from(path)
    // };

    let cli = Cli::parse();

    let content = std::fs::read_to_string(&cli.path).map_err(|_| format!("Error while Reading {}", cli.path.display()))?;


    println!("File content is {}", content);

    for line in content.lines() {
        if line.contains(&cli.pattern) {
            println!("{}", line)
        }
    }

    println!("Pattern {:?}, path {:?}", cli.pattern, cli.path);

    Ok(())
}
