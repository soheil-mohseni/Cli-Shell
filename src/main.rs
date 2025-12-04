use std::env;
use std::fs;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::path::Path;
use std::sync::OnceLock;
use std::{collections::HashMap, io::stdin};

struct Engine {
    handlers: HashMap<&'static str, fn(&str)>,
}

static ENGINE: OnceLock<Engine> = OnceLock::new();

impl Engine {
    fn new() -> Self {
        let mut handlers: HashMap<&'static str, fn(&str)> = HashMap::new();

        handlers.insert("echo", Engine::echo);
        handlers.insert("exit", Engine::exit);
        handlers.insert("type", Engine::type_command);

        Self { handlers }
    }
    fn echo(command: &str) {
        println!("{}", command);
    }
    fn type_command(command: &str) {
        let eng = ENGINE.get().unwrap();
        if let Some(_) = eng.handlers.get(command) {
            println!("{} is a shell builtin", command);
        } else {
            let res = executable_path(command);
            match res {
                Ok(response) => {
                    println!("{command} is {response}");
                }
                Err(err) => {
                    eprintln!("{}: not found", command);
                }
            }
        }
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
    // ENGINE.set(engine_instance).unwrap();

    let engine_instance = ENGINE.get_or_init(|| Engine::new());

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

fn executable_path(exec_name: &str) -> Result<String, &str> {
    use std::os::unix::fs::PermissionsExt;
    let path_var = env::var("PATH").unwrap();
    let path_list = path_var.split(":");
    let mut found = false;
    let mut last_path = "";
    for path in path_list {
        let full_path = Path::new(path).join(exec_name);
        if let Ok(meta) = fs::metadata(&full_path) {
            let mode = meta.permissions().mode();
            if mode & 0o100 != 0 {
                found = true;
                last_path = path;
            }
        }
    }
    if found == true {
        return Ok(format!("{}/{}", last_path, exec_name));
    }
    return Err("not found");
}
