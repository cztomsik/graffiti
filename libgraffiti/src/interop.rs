/// Some of the data can be exchanged with nodejs
/// and/or other platforms
///
/// Each platform should implement this for basic types
/// and then provide `interop!` macro & include the contents
/// of `./generated.rs` which in turn calls this macro
/// to generate structs, enums & tagged unions
pub trait Interop<T> {
    fn from_external(external: T) -> Self;
    fn to_external(self) -> T;
}

#[cfg(not(target_arch = "wasm32"))]
mod nodejs;

// TODO: wasm
