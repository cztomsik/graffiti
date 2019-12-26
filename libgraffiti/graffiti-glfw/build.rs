use cc::Build;

// based on https://github.com/glfw/glfw/blob/master/src/CMakeLists.txt
fn main() {
    let mut build = Build::new();

    // no warns
    build.flag("-w");

    // define platform first
    #[cfg(target_os="macos")]
    build.define("_GLFW_COCOA", Some("1"));

    #[cfg(target_os="linux")]
    build.define("_GLFW_X11", Some("1"));

    // shared
    build
        .file("glfw/src/context.c")
        .file("glfw/src/init.c")
        .file("glfw/src/input.c")
        .file("glfw/src/monitor.c")
        .file("glfw/src/vulkan.c")
        .file("glfw/src/window.c")
    ;

    #[cfg(target_os="macos")]
    build
        .file("glfw/src/cocoa_init.m")
        .file("glfw/src/cocoa_joystick.m")
        .file("glfw/src/cocoa_monitor.m")
        .file("glfw/src/cocoa_window.m")
        .file("glfw/src/cocoa_time.c")
        .file("glfw/src/posix_thread.c")
        .file("glfw/src/nsgl_context.m")
        .file("glfw/src/egl_context.c")
        .file("glfw/src/osmesa_context.c")
    ;

    #[cfg(target_os="linux")]
    build
      // TODO: wayland
      .file("glfw/src/x11_init.c")
      .file("glfw/src/x11_monitor.c")
      .file("glfw/src/x11_window.c")

      .file("glfw/src/xkb_unicode.c")
      .file("glfw/src/posix_time.c")
      .file("glfw/src/posix_thread.c")
      .file("glfw/src/glx_context.c")
      .file("glfw/src/egl_context.c")
      .file("glfw/src/osmesa_context.c")

      .file("glfw/src/linux_joystick.c")
    ;

    // build lib
    build
        .compile("libglfw3.a")
    ;
}
