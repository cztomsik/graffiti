use std::os::raw::{c_int, c_float, c_uchar, c_void};

extern "C" {
    pub fn stbi_loadf_from_memory(buffer: *const c_uchar, len: c_int, x: *mut c_int, y: *mut c_int, comp: *mut c_int, req_comp: c_int) -> *mut c_float;

    pub fn stbi_image_free(img_data: *mut c_void);
}
