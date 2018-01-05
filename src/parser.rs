use std::process::Command;
use std::fs;
use std::env;
use std::path::Path;
use std::io;
use std::io::Write;

use prompt::Prompt;

#[derive(Debug, Clone)]
pub struct CommandTable {
    cmd: String,
    args: Option<Vec<String>>,
}

impl CommandTable {
    fn is_builtin(self) -> bool {
        let builtins: Vec<&str> = vec!["for", "cd", "ls"];
        let x: Vec<&str> = builtins
            .into_iter()
            .filter(|b| b.to_string() == self.cmd)
            .collect();
        if x.len() != 0 {
            true
        } else {
            false
        }
    }

    pub fn parse(input: String) {
        let cclone = input.clone();
        // cmd_parse collect the cmd in a vec
        let mut cmd_parse: Vec<&str> = cclone.trim().split(" ").collect();
        // split the first cmd off and leave vec of args
        let ct: CommandTable = if cmd_parse.len() == 1 {
            CommandTable {
                cmd: cmd_parse[0].to_string(),
                args: None,
            }
        } else {
            let cmd = cmd_parse.split_off(0);
            let args: Vec<String> = cmd_parse.iter().map(|b| b.to_string()).collect();
            CommandTable {
                cmd: cmd[0].to_string(),
                args: Some(args),
            }
        };
        // check if its a builtin
        let c = ct.clone();
        if ct.is_builtin() {
            println!("builtin")
        } else {
            if c.args.is_some() {
                println!("{:?}", &c);
                let p = Command::new(c.cmd)
                    .args(&c.args.unwrap())
                    .current_dir(Prompt::get_cur_dir())
                    .output()
                    .unwrap();
            } else if c.cmd == "" {
                print!("");
            } else {
                let p = Command::new(c.cmd)
                    .current_dir(Prompt::get_cur_dir())
                    .output()
                    .unwrap();
                if p.status.code().unwrap() != 0 {
                    print!("{}", String::from_utf8_lossy(&p.stderr));
                } else {
                    print!("{}", String::from_utf8_lossy(&p.stdout));
                }
            }
            io::stdout().flush();
        }
    }
}
