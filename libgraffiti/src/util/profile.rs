// super-simple profiling
// x reports how long it took since previous call
// x if there is no message, it just resets marker

use std::cell::Cell;
use std::time::Instant;

#[cfg(feature = "profile")]
thread_local! {
    pub(crate) static LAST_TIME: Cell<Instant> = Cell::new(Instant::now());
}

macro_rules! profile {
    ($($text: literal)*) => {
        #[cfg(feature = "profile")]
        crate::util::LAST_TIME.with(|last| {
            let prev = last.replace(std::time::Instant::now());
            $(println!("{} took {}s", $text, prev.elapsed().as_secs_f32()))*;
        })
    };
}
