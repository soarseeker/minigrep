use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(&args);// this was fixed with the ampersand, but it was not a part of the instruction.
    // it was just used to diplay the parameter values in step one
    // thus ownship was transferred to dbg without the ampersand

    // this line used a function to parse the arguments
    // let config = parse_config(&args);

    // this was the config assignment before returning and error result
    //let config = Config::new(&args);

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

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
// this is the implimentation for config when build is used
impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

// this was configs implimentation when panic was used
/* impl Config {
    fn new(args: &[String]) -> Result< Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

       Ok( Config { query, file_path })
    }
} */
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