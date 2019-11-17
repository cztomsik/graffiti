use graffiti::ffi::*;

// nvm use 11
// ./emsdk install latest
// ./emsdk activate latest
// source ./emsdk_env.sh 
// npm run build -- --target wasm32-unknown-emscripten --bin wasm
pub fn main() {
    gft_init();

    println!("hello")
}
