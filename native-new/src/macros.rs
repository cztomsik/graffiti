// very basic logging to save some deps
// for now it's enabled/disabled by (un)commenting the line
//
// TODO: auto-enable based on some env var
macro_rules! silly {
    ($($arg:tt)+) => (
        // println!($($arg)+);
    )
}

macro_rules! debug {
    ($($arg:tt)+) => (
        println!($($arg)+);
    )
}

macro_rules! error {
    ($($arg:tt)+) => (
        println!($($arg)+);
    )
}
