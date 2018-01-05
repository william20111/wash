
use std::env;

pub struct Prompt {
    user: String,
    cwd: String,
    hostname: String,
}

impl Prompt {
    pub fn create_prompt() -> Prompt {
        Prompt {
            user: Prompt::get_user(),
            cwd: Prompt::get_cur_dir(),
            hostname: Prompt::get_hostname(),
        }
    }

    pub fn display() -> String {
        let p = Prompt::create_prompt();
        format!("{}@{}:{}>", p.user, p.hostname, p.cwd)
    }

    //pub fn update_prompt(&self, dir: String) -> String {}

    fn get_user() -> String {
        env::var("USERNAME").unwrap()
    }

    pub fn get_cur_dir() -> String {
        env::current_dir().unwrap().to_str().unwrap().to_string()
    }

    fn get_hostname() -> String {
        env::var("HOSTNAME").unwrap()
    }
}
