extern crate wash;

use std::io;
use std::io::Write;
use std::path::Path;
use std::thread;
use std::error::Error;
use std::env;
use std::fs;

use wash::prompt::Prompt;
use wash::history::History;
use wash::parser;

fn main() {
    // setup history
    //let mut x: Vec<i8> = vec![1, 2, 3, 4, 5, 6];
    //let x2 = x.split_off(1);
    //println!("x {:?} , x2 {:?}", x, x2);

    let history_file = History::setup_history();
    let exit_code: i8 = 0;
    let mut directory = Prompt::setup_prompt();
    // main shell loop
    loop {
        // buf for input
        let mut input = String::new();
        // update prompt
        let directory = directory.update();
        // display prompt
        print!("{} ", directory.display());
        io::stdout().flush();
        // READLINE
        io::stdin().read_line(&mut input);
        // write history
        History::write_history(history_file.clone(), input.clone());
        parser::CommandTable::parse(input);
    }
}
