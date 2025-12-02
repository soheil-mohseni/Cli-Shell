#[allow(unused_imports)]
use std::io::{self, Write};
use std::{collections::HashMap, io::stdin};

struct Engine {
    handlers: HashMap<&'static str, fn(&str)>,
}

// #[derive(Hash, Eq, PartialEq)]
// enum Engine_method {
//     echo,
// }

impl Engine {
    fn new() -> Self {
        let mut handlers: HashMap<&'static str, fn(&str)> = HashMap::new();

        handlers.insert("echo", Engine::echo);
        handlers.insert("exit", Engine::exit);

        Self { handlers }
    }
    fn echo(command: &str) {
        println!("{}", command);
    }

    fn exit(_: &str) {
        std::process::exit(0);
    }
    fn dispatch(&self, cmd: &str, command_value: &str) {
        if let Some(handler) = self.handlers.get(cmd) {
            handler(command_value);
        } else {
            eprintln!("{}: command not found", cmd);
        }
    }
}

fn main() {
    let engine_instance = Engine::new();
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        let _ = stdin().read_line(&mut input).unwrap();
        let mut command_input = input.trim().splitn(2, " ");
        let command = command_input.next().unwrap_or("");
        let command_value: &str = command_input.next().unwrap_or("");
        engine_instance.dispatch(command, command_value);
    }
}
