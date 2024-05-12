use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", file_path);

    let file_contents = fs::read_to_string(file_path).expect("Expected to read the file");

    println!("Contents of the file:\n{}", file_contents);
}
