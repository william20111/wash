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
    let history_file = History::setup_history();
    let exit_code: i8 = 0;
    let mut directory = Prompt::create_prompt();
    // main shell loop
    loop {
        // buf for input
        let mut input = String::new();
        // update prompt
        let prompt = Prompt::display();
        // display prompt
        print!("{} ", prompt);
        io::stdout().flush();
        // READLINE
        io::stdin().read_line(&mut input);
        // write history
        History::write_history(history_file.clone(), input.clone());
        parser::CommandTable::parse(input);
    }
}
