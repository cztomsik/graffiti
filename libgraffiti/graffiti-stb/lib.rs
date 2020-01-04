// this is temporary, we rather should try to detect & use one of the system-available libraries
// but maybe we can keep it as compile option or something
//
// stb is nice but it's not meant to be feature-complete and if you're
// loading images from uncontrolled sources, it might also become a security issue

use std::os::raw::{c_int, c_float, c_uchar, c_void};

extern "C" {
    pub fn stbi_loadf_from_memory(buffer: *const c_uchar, len: c_int, x: *mut c_int, y: *mut c_int, comp: *mut c_int, req_comp: c_int) -> *mut c_float;

    pub fn stbi_image_free(img_data: *mut c_void);
}
