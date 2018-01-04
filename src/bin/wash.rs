extern crate wash;

use std::io;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::thread;
use std::error::Error;
use std::env;
use std::fs;

use wash::prompt::Prompt;
use wash::shell;
use wash::history::History;
use wash::parser;

fn main() {
    // setup history
    let history_file = History::setup_history();
    
    // main shell loop
    loop {
        // buf for input
        let mut input = String::new();
        // display prompt
        print!("{} ", Prompt::display());
        io::stdout().flush();
        // READLINE
        io::stdin().read_line(&mut input);
        // write history
        History::write_history(history_file.clone(), input.clone());  
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
        }
    }
}
