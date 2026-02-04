use std::env;
use std::fs;
use std::process;
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    show_current_dir();

        if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);}
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{contents}");

    Ok(())
}

struct Config {
    _query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let _query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { _query, file_path })
    }
}

fn show_current_dir() {
    match env::current_dir() {
        Ok(path) => {
            // Convert PathBuf to a displayable string
            println!("Current directory: {}", path.display());
        }
        Err(e) => {
            eprintln!("Failed to get current directory: {}", e);
        }
    }
}
