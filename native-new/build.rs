use std::fs;

static DIR: &str = "generated";

fn main() {
    let _ = fs::remove_dir_all(DIR);
    let _ = fs::create_dir(DIR);
}
