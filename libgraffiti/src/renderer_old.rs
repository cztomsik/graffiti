impl Renderer {
    fn render_frame(&mut self, frame: &mut Frame) {
        unsafe {
            for b in &frame.batches {
                match *b {
                    Batch::Rects { opaque, num: _ } => {
                        // depth/alpha
                        if opaque {
                            gl::Enable(gl::DEPTH_TEST);
                            gl::Disable(gl::BLEND);
                        } else {
                            gl::Disable(gl::DEPTH_TEST);
                            gl::Enable(gl::BLEND);
                            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
                            gl::BlendEquation(gl::FUNC_ADD);
                        }

                        gl::UseProgram(self.rect_program);
                        frame.rects.bind_to(gl::ARRAY_BUFFER);
                        frame.indices.bind_to(gl::ELEMENT_ARRAY_BUFFER);
                        gl::EnableVertexAttribArray(0);
                        gl::VertexAttribPointer(
                            0,
                            2,
                            gl::FLOAT,
                            gl::FALSE,
                            (mem::size_of::<Vertex<Color>>()) as GLint,
                            0 as *const GLvoid,
                        );
                        gl::EnableVertexAttribArray(1);
                        gl::VertexAttribPointer(
                            1,
                            4,
                            gl::UNSIGNED_BYTE,
                            gl::FALSE,
                            (mem::size_of::<Vertex<Color>>()) as GLint,
                            (mem::size_of::<Pos>()) as *const std::ffi::c_void,
                        );

                        gl::DrawElements(gl::TRIANGLES, frame.indices.data.len() as i32, gl::UNSIGNED_SHORT, std::ptr::null());
                        check();

                        // setup for alpha (depth, alpha, buffers)
                        gl::Disable(gl::DEPTH_TEST);
                        gl::Enable(gl::BLEND);
                        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
                        gl::BlendEquation(gl::FUNC_ADD);

                        // gl::DrawElements(gl::TRIANGLES, vertices_count as i32, gl::UNSIGNED_SHORT, (offset * std::mem::size_of::<VertexIndex>()) as *const std::ffi::c_void);
                    }

                    Batch::Image => debug!("TODO: render image"),
                    Batch::Text => debug!("TODO: render text"),
                }
            }
        }
    }
}


// low-level stuff, merged (and improved) from PoC in cztomsiK/new-hope

/// Some things are shared/cached across multiple frames (texts) and some
/// are rebuilt every frame (rects) for the sake of simplicity.
