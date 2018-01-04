extern crate wash;

use std::io;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::thread;
use std::error::Error;
use std::env;
use std::fs;

fn main() {
    // init history
    let home = env::home_dir().unwrap();
    let path = Path::new(".jshell_history");
    let history_file = format!("{}/{}", home.to_str().unwrap(), path.to_str().unwrap());
    let mut history = fs::File::open(history_file).unwrap();
    // main shell loop
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input);
        if input.starts_with("cd") || input.starts_with("ls") {
            match input.as_str() {
                "cd\n" => {
                    let cmd: Vec<&str> = input.trim().split(" ").collect();
                    match env::set_current_dir(&cmd[1]) {
                        Ok(_) => {}
                        Err(e) => println!("{}", e),
                    }
                }
                "ls\n" => {
                    let cmd: Vec<&str> = input.trim().split(" ").collect();
                    if cmd.len() > 0 {
                        let paths = fs::read_dir("./");
                        for path in paths {
                            println!("{:?}", path);
                        }
                    } else {
                        let paths = fs::read_dir(cmd[1]);
                        for path in paths {
                            println!("{:?}", path);
                        }
                    }
                }
                _ => {}
            }
        } else {
            Command::new(&input);
            match history.write_all(input.as_bytes()) {
                Err(why) => {
                    println!("Failed to write to {}: {}",
                             path.display(),
                             why.description())
                }
                Ok(_) => {}
            }
        }
    }
}
