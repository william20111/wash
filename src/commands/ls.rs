use std::fs;

pub fn exec(dir: String) {
    let paths = fs::read_dir("./").unwrap();

    for path in paths {
        println!("{}", path.unwrap().path().display())
    }
}
