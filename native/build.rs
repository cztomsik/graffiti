fn main() {
    if cfg!(windows) {
        println!("cargo:rustc-link-search={}", "C:\\Program Files\\nodejs");
        println!("cargo:rustc-link-lib={}", "node");
    }
}
