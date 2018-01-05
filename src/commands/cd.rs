use std::fs;
use std::env;

pub fn exec(dir: String) {
    env::set_current_dir(dir);
}
