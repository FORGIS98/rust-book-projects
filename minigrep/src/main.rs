use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    // &args[0] = program name
    let query = &args[1];
    let filename = &args[2];

    let file_content = fs::read_to_string(filename).expect("Error reading the file.");

    println!("Here is the content:\n\n{}", file_content);
}
