use std::env; // To read command-line args

fn main() {
    let args: Vec<String> = env::args().collect();

    // &args[0] = program name
    let query = &args[1];
    let filename = &args[2];

    println!("Search for `{}` in `{}`", query, filename);
}