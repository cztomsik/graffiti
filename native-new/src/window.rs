use crate::api::{
    Scene, Window, WindowEvent,
};
use crate::render::{SceneRenderer, WebrenderRenderer};
use crate::scene::ArrayScene;
use gleam::gl::GlFns;
use glfw::{Context, Window as GlfwWindow};
use crate::generated::SurfaceId;

pub struct AppWindow {
    glfw_window: GlfwWindow,
    scene: ArrayScene,
    renderer: WebrenderRenderer,
    // TODO: size (so we can resize)
    mouse_pos: (f32, f32)
}

impl AppWindow {
    pub fn new(mut glfw_window: GlfwWindow) -> Self {
        let gl = unsafe { GlFns::load_with(|addr| glfw_window.get_proc_address(addr)) };

        let window = AppWindow {
            glfw_window,
            scene: ArrayScene::new(),
            renderer: WebrenderRenderer::new(gl),
            mouse_pos: (0., 0.)
        };

        window
    }

    // TODO
    pub fn handle_event(&mut self, event: glfw::WindowEvent) -> Option<WindowEvent> {
        // TODO: we don't need Option currently so maybe we can remove it in the future
        match event {
            event => Some(match event {
                glfw::WindowEvent::CursorPos(x, y) => {
                    let x = x as f32;
                    let y = y as f32;

                    self.mouse_pos = (x, y);

                    WindowEvent::MouseMove { target: self.hit_test() }
                }
                glfw::WindowEvent::Scroll(delta_x, delta_y) => {
                    self.scroll((delta_x as f32, delta_y as f32));

                    WindowEvent::Scroll { target: self.hit_test() }
                }
                glfw::WindowEvent::MouseButton(_button, action, _modifiers) => {
                    let target = self.hit_test();

                    match action {
                        glfw::Action::Press => WindowEvent::MouseDown { target },
                        glfw::Action::Release => WindowEvent::MouseUp { target },
                        _ => unreachable!("mouse should not repeat"),
                    }
                },
                //glutin::WindowEvent::ReceivedCharacter(ch) => WindowEvent::KeyPress(ch as u16),
                //glutin::WindowEvent::CloseRequested => WindowEvent::Close,
                //glutin::WindowEvent::Resized(..) => WindowEvent::Resize,
                _ => WindowEvent::Unknown,
            }),
        }
    }

    fn hit_test(&self) -> SurfaceId {
        let (x, y) = self.mouse_pos;

        self
            .renderer
            .hit_test(x, y)
            // for any window event, there's always hit (root surface at least) because it's somewhere inside
            // we need to send some MouseMove event because of onMouseOut (prevTarget !== target)
            // TODO: should be a const or something
            .unwrap_or(0)
    }

    fn scroll(&mut self, delta: (f32, f32)) {
        self.glfw_window.make_current();
        self.renderer.scroll(self.mouse_pos, delta);
        self.glfw_window.swap_buffers();
    }
}

impl Window for AppWindow {
    fn scene_mut(&mut self) -> &mut Scene {
        &mut self.scene
    }

    fn render(&mut self) {
        // TODO: set on resize
        let layout_size = self.renderer.layout_size;
        self.scene.set_layout_size(layout_size.width, layout_size.height);
        self.scene.calculate_layout();

        self.glfw_window.make_current();
        self.renderer.render(&self.scene);
        self.glfw_window.swap_buffers();
    }
}
