fn main() {
    // auto-enable nodejs bindings for `npm run ...`
    // https://docs.npmjs.com/cli/run-script
    //
    // note we can't add any extra deps this way
    // https://github.com/rust-lang/cargo/issues/5499
    if std::env::var("NODE").is_ok() {
        println!("cargo:rustc-cfg=feature=\"nodejs\"");
    }
}
