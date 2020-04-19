// this is temporary, we should rather try to detect & use one of the system-available libraries
// but maybe we can keep it as compile option or something
//
// stb is nice but it's not feature-complete nor safe and loading resources
// from untrusted sources might be a security issue

use std::os::raw::{c_int, c_float, c_uchar, c_void};

// -- image

extern "C" {
    pub fn stbi_loadf_from_memory(buffer: *const c_uchar, len: c_int, x: *mut c_int, y: *mut c_int, comp: *mut c_int, req_comp: c_int) -> *mut c_float;

    pub fn stbi_image_free(img_data: *mut c_void);
}


// -- truetype

extern "C" {
    pub fn stbtt_InitFont(font: *mut stbtt_fontinfo, data: *const u8, offset: usize) -> c_int;

    pub fn stbtt_FindGlyphIndex(font: *const stbtt_fontinfo, codepoint: c_int) -> c_int;

    pub fn stbtt_GetGlyphBox(font: *const stbtt_fontinfo, glyph_index: c_int, x0: *mut c_int, y0: *mut c_int, x1: *mut c_int, y1: *mut c_int);

    pub fn stbtt_GetGlyphHMetrics(font: *const stbtt_fontinfo, glyph_index: c_int, advance: *mut c_int, left_side_bearing: *mut c_int);

    pub fn stbtt_ScaleForPixelHeight(font: *const stbtt_fontinfo, height: f32) -> f32;
}

#[repr(C)]
pub struct stbtt_fontinfo([*const c_void; 2], [c_int; 12], [stbtt__buf; 6]);

#[repr(C)]
pub struct stbtt__buf(*const c_void, c_int, c_int);
