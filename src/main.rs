use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(&args);// this was fixed with the ampersand, but it was not a part of the instruction.
    // it was just used to diplay the parameter values in step one
    // thus ownship was transferred to dbg without the ampersand

    // this line used a function to parse the arguments
    // let config = parse_config(&args);

    let config = Config::new(&args);

    show_current_dir();

    //when implimenting the struct, the file_path needed config. place in front of file_path
    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file"); 
    
//this broke when moving from a function to a struct implimentation
/*     let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file"); */

    println!("With text:\n{contents}");
}
struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("not enough arguments");}
        let query = args[1].clone();
        let file_path = args[2].clone();

        Config { query, file_path }
    }
}
// this was the function to parse the arguments
/* fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();
    println!("Searching for {query}");
    println!("In file {file_path}");

    Config { query, file_path }

}
 */
fn show_current_dir(){
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