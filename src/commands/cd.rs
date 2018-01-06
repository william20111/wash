use std::fs;
use std::env;

pub fn exec(dir: String) {
    match env::set_current_dir(dir) {
        Ok(_) => {},
        Err(e) => {println!("{}", e);}
    }
}
