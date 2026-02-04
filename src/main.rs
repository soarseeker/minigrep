use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(&args);// this was fixed with the ampersand, but it was not a part of the instruction.
    // it was just used to diplay the parameter values in step one
    // thus ownship was transferred to dbg without the ampersand

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {query}");
    println!("In file {file_path}");
    match env::current_dir() {
        Ok(path) => {
            // Convert PathBuf to a displayable string
            println!("Current directory: {}", path.display());
        }
        Err(e) => {
            eprintln!("Failed to get current directory: {}", e);
        }
    }



    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}
