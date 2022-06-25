fn main() {
    // we can't easily locate nodejs headers and they
    // might not be present at all so instead we do weak-linkage
    //
    // TODO: maybe we should first try?
    #[cfg(feature = "nodejs")]
    {
        // TODO: check correct linker args on each platform
        // TODO: wasm
        // #[cfg(target_os = "linux")]
        // #[cfg(target_os = "windows")]
        // #[cfg(target_family = "unix")]
        #[cfg(target_os = "macos")]
        {
            println!("cargo:rustc-link-arg=-undefined");
            println!("cargo:rustc-link-arg=dynamic_lookup");
        }
    }
}
