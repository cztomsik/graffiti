use crate::commons::{Au, Pos};
use crate::app::{WindowEvent};
use crate::platform::NativeWindow;
use graffiti_glfw::*;
use std::ptr;
use std::os::raw::{c_int, c_uint, c_double, c_void};

pub unsafe fn init() {
    assert_eq!(glfwInit(), GLFW_TRUE, "init GLFW");

    #[cfg(target_os="macos")] {
        glfwInitHint(GLFW_COCOA_CHDIR_RESOURCES, GLFW_FALSE);

        glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 3);
        glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 2);
        glfwWindowHint(GLFW_OPENGL_FORWARD_COMPAT, GLFW_TRUE);
        glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);
    }
}

pub unsafe fn get_events(poll: bool) {
    if poll {
        glfwPollEvents()
    } else {
        // wait a bit otherwise (save battery)
        //
        // this number limits node.js responsivity
        // lower means sooner handling of I/O & timers
        // at the expense of some extra CPU overhead
        //
        // higher it is, more laggy it might feel
        // (http responses "taking too long", etc.)
        //
        // this number should be fine unless somebody is animating
        // with setTimeout or some other bad things
        //
        // ideally, we should just block with glfwWaitEvents()
        // but that would need somehow to send glfwPostEmptyEvent()
        // if anything in node.js is ready (not just I/O but also timers)
        // and it's not yet obvious to me how that could be done
        // so this is definitely good enough for now
        glfwWaitEventsTimeout(0.15);
    }
}

pub unsafe fn create_window(title: &str, width: i32, height: i32) -> NativeWindow {
    let w = glfwCreateWindow(width, height, c_str!(title), ptr::null_mut(), ptr::null_mut());
    assert!(!w.is_null(), "create GLFW window");

    glfwMakeContextCurrent(w);
    gl::load_with(|addr| glfwGetProcAddress(c_str!(addr)));

    glfwSetCursorPosCallback(w, handle_glfw_cursor_pos);
    glfwSetScrollCallback(w, handle_glfw_scroll);
    glfwSetMouseButtonCallback(w, handle_glfw_mouse_button);
    glfwSetCharCallback(w, handle_glfw_char);
    glfwSetKeyCallback(w, handle_glfw_key);
    glfwSetWindowSizeCallback(w, handle_glfw_window_size);
    glfwSetFramebufferSizeCallback(w, handle_glfw_framebuffer_size);
    glfwSetWindowCloseCallback(w, handle_glfw_window_close);

    // VSYNC=0 to disable
    let vsync = std::env::var("VSYNC").map(|s| s.parse().expect("vsync number")).unwrap_or(1);
    glfwSwapInterval(vsync);

    w as *mut c_void
}

pub unsafe fn make_current(native_window: NativeWindow) {
    glfwMakeContextCurrent(native_window as *mut GlfwWindow)
}

pub unsafe fn detach_current() {
    glfwMakeContextCurrent(ptr::null_mut());
}

pub unsafe fn swap_buffers(native_window: NativeWindow) {
    glfwSwapBuffers(native_window as *mut GlfwWindow)
}

unsafe extern "C" fn handle_glfw_cursor_pos(w: *mut GlfwWindow, x: c_double, y: c_double) {
    window_event!(w, w.mouse_move(Pos::new(x as Au, y as Au)))
}

unsafe extern "C" fn handle_glfw_scroll(_w: *mut GlfwWindow, _: c_double, _: c_double) {
    error!("TODO: handle_glfw_scroll");
}

unsafe extern "C" fn handle_glfw_mouse_button(w: *mut GlfwWindow, _button: c_int, action: c_int, _mods: c_int) {
    window_event!(w, match action {
        GLFW_PRESS => w.mouse_down(),
        GLFW_RELEASE => w.mouse_up(),
        _ => unreachable!("press/release expected"),
    })
}

unsafe extern "C" fn handle_glfw_key(w: *mut GlfwWindow, key: c_int, _scancode: c_int, action: c_int, _mods: c_int) {
    let key_code = get_key_code(key);

    window_event!(w, match action {
        // TODO: repeat works for some keys but for some it doesn't
        // not sure if it's specific for mac (special chars overlay)
        GLFW_RELEASE => w.key_up(key_code),
        _ => w.key_down(key_code),
    })
}

unsafe extern "C" fn handle_glfw_char(w: *mut GlfwWindow, char: c_uint) {
    window_event!(w, w.key_press(char as u16))
}

unsafe extern "C" fn handle_glfw_window_size(w: *mut GlfwWindow, width: c_int, height: c_int) {
    window_event!(w, w.resize((width as f32, height as f32)));
    glfwSwapBuffers(w);
}

unsafe extern "C" fn handle_glfw_framebuffer_size(_w: *mut GlfwWindow, width: c_int, height: c_int) {
    gl::Viewport(0, 0, width, height);
}

unsafe extern "C" fn handle_glfw_window_close(w: *mut GlfwWindow) {
    window_event!(w, w.close())
}

// from glfw to js `e.which`
// TODO: modifier (left/right for shift/ctrl/alt)
fn get_key_code(key: c_int) -> u16 {
    (match key {
        // some codes are the same
        32 | 48 ..= 57 | 65 ..= 90 => key,
        91 ..= 93 => key + 128,

        44 => 188,
        45 => 187,
        46 => 190,
        47 => 189,
        59 => 186,
        61 => 187,
        256 => 27,
        257 => 13,
        258 => 9,
        259 => 8,
        262 => 39,
        263 => 37,
        264 => 40,
        265 => 38,

        // TODO: -1, APOSTROPHE, GRAVE_ACCENT, WORLD1, WORLD2, INSERT, DELETE,
        //   PAGE_UP, PAGE_DOWN, HOME, END, CAPS_LOCK, SCROLL_LOCK, NUM_LOCK
        //   PRINT_SCREEN, PAUSE, F1-F25, KP0-KP9, KP_*
        //   LEFT/RIGHT_SHIFT/CONTROL/ALT
        _ => 0
    }) as u16
}
