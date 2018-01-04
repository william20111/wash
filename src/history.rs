
use std::fs;
use std::io::Write;
use std::io;
use std::env;
use std::thread;
use std::fs::OpenOptions;
use std::time::{SystemTime, UNIX_EPOCH};


pub struct History {
    file: String,
}

impl History {
    pub fn setup_history() -> String {
        let home = env::home_dir().unwrap();
        format!("{}/{}", home.to_str().unwrap(), ".wash_history")
    }

    pub fn write_history(hf: String, input: String) {
        thread::spawn(move || {
            let mut history = OpenOptions::new()
            .append(true)
            .create(true)
            .open(hf).unwrap();
            match SystemTime::now().duration_since(UNIX_EPOCH) {
                Ok(n) => {
                    let h = format!("{}: {}", n.as_secs(), input);
                    history.write(h.as_bytes());
                }
                Err(e) => {},
            }
        });
    }
}