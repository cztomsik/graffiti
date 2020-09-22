pub trait Painter {
    fn fill_rect(&mut self, left: f32, top: f32, width: f32, height: f32);
    fn draw_text(&mut self, left: f32, top: f32, text: &str);
}

use std::sync::mpsc::{Receiver, Sender};

// useful for testing
impl Painter for Sender<String> {
    fn fill_rect(&mut self, left: f32, top: f32, width: f32, height: f32) {
        self.send(format!("fill_rect({:?}, {:?}, {:?}, {:?})", left, top, width, height));
    }

    fn draw_text(&mut self, left: f32, top: f32, text: &str) {
        self.send(format!("draw_text({:?}, {:?}, {:?})", left, top, text));
    }
}

use std::ffi::c_void;

// TODO
unsafe impl Send for Surface {}

// TODO: trait?
pub struct Surface {
    glfw_window: *mut c_void,
    size: (f32, f32),
    vg: *mut NVGcontext,
}

impl Surface {
    pub(crate) fn new(glfw_window: *mut c_void, size: (f32, f32)) -> Self {
        let vg = unsafe {
            let vg = nvgCreateGL2((NVGcreateFlags::NVG_ANTIALIAS | NVGcreateFlags::NVG_STENCIL_STROKES | NVGcreateFlags::NVG_DEBUG).bits());

            let FONT = include_bytes!("../resources/Roboto/Roboto-Regular.ttf");

            nvgCreateFontMem(vg, c_str!("sans"), FONT as *const _ as *mut u8, FONT.len() as i32, 0);

            vg
        };

        Self { glfw_window, size, vg }
    }

    pub fn size(&self) -> (f32, f32) {
        self.size
    }

    pub fn paint(&mut self, paint_fn: impl Fn(&mut Painter)) {
        //glfwGetWindowSize(window, &win_width, &win_height);
        //glfwGetFramebufferSize(window, &fb_width, &fb_height);
        //px_ratio = fb_width / win_width as f32;
        //glViewport(0, 0, fb_width, fb_height);

        let (win_width, win_height) = (800., 600.);
        let px_ratio = 1.;

        let vg = self.vg;

        unsafe {
            graffiti_glfw::glfwMakeContextCurrent(self.glfw_window);

            gl::ClearColor(0.5, 0., 0., 1.);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT);

            nvgBeginFrame(vg, win_width, win_height, px_ratio);

            paint_fn(&mut NanoVgPainter { vg });

            nvgEndFrame(vg);

            graffiti_glfw::glfwSwapBuffers(self.glfw_window);
        }
    }
}

use nanovg_sys::*;

pub struct NanoVgPainter {
    vg: *mut c_void,
}

impl Painter for NanoVgPainter {
    fn fill_rect(&mut self, left: f32, top: f32, width: f32, height: f32) {
        let vg = self.vg;

        unsafe {
            nvgBeginPath(vg);
            nvgRect(vg, left, top, width, height);
            nvgFillColor(vg, nvgRGBA(255, 127, 255, 10));
            nvgFill(vg);
        }
    }

    fn draw_text(&mut self, left: f32, top: f32, text: &str) {
        let vg = self.vg;

        unsafe {
            let ptr = text.as_ptr() as _;

            nvgFontSize(vg, 14.0);
            nvgFontFace(vg, c_str!("sans"));
            nvgTextAlign(vg, (NVGalign::NVG_ALIGN_LEFT | NVGalign::NVG_ALIGN_MIDDLE).bits());
            nvgFillColor(vg, nvgRGBA(255, 255, 255, 128));
            nvgText(vg, left, top, ptr, ptr.add(text.len()));
        }
    }
}
