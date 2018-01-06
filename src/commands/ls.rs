use std::fs;

pub fn exec(dir: String) {
    let paths = fs::read_dir("./").unwrap();
    let mut files = String::new();
    for path in paths {
        //println!("{}", path.unwrap().path().display())
        files.push_str(path.unwrap().path().file_name().unwrap().to_str().unwrap());
        files.push_str("  ");
    }
    println!("{}", files)
}
