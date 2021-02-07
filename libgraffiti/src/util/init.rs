// run some code when the lib is loaded
// - one per module
// - order is not specified
macro_rules! init {
    ($($body:tt)*) => {
        #[used]
        #[no_mangle]
        #[cfg_attr(target_os = "linux", link_section = ".ctors")]
        #[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
        #[cfg_attr(target_os = "windows", link_section = ".CRT$XCU")]
        static __INIT: extern "C" fn() = {
            extern "C" fn init() {
                $($body)*
            }

            init
        };
    };
}
