// debug logging
// (run cargo with `--features silly/debug`)

macro_rules! debug {
    ($($arg:tt)+) => (
        #[cfg(any(feature = "debug", feature = "silly"))]
        println!($($arg)+);
    )
}

macro_rules! silly {
    ($($arg:tt)+) => (
        #[cfg(feature = "silly")]
        println!($($arg)+);
    )
}
