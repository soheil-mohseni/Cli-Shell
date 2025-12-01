use std::io::stdin;
#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // TODO: Uncomment the code below to pass the first stage
    print!("$ ");
    let mut command  = String::new();
    let _ = stdin().read_line(&mut command).unwrap();
    eprintln!("{}: Invalid commadnd",command.trim());
    io::stdout().flush().unwrap();
}
