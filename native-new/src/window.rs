use crate::api::{
    Scene, Window, WindowEvent,
};
use crate::render::{SceneRenderer, WebrenderRenderer};
use crate::scene::ArrayScene;
use gleam::gl::GlFns;
use glfw::{Context, Window as GlfwWindow};

pub struct AppWindow {
    glfw_window: GlfwWindow,
    scene: ArrayScene,
    renderer: WebrenderRenderer,
    // TODO: size (so we can resize)
    // TODO: mouse x,y (so we can do webrender.scroll(x, y, delta_x, delta_y))
}

impl AppWindow {
    pub fn new(mut glfw_window: GlfwWindow) -> Self {
        let gl = unsafe { GlFns::load_with(|addr| glfw_window.get_proc_address(addr)) };

        let window = AppWindow {
            glfw_window,
            scene: ArrayScene::new(),
            renderer: WebrenderRenderer::new(gl),
        };

        window
    }

    // TODO
    pub fn translate_event(&self, event: glfw::WindowEvent) -> Option<WindowEvent> {
        // TODO: we don't need Option currently so maybe we can remove it in the future
        match event {
            event => Some(match event {
                glfw::WindowEvent::CursorPos(x, y) => {
                    // for any window event, there's always hit (root surface at least) because it's somewhere inside
                    // we need to send some MouseMove event because of onMouseOut (prevTarget !== target)
                    let target = self
                        .renderer
                        .hit_test(x as f32, y as f32)
                        // TODO: should be a const or something
                        .unwrap_or(0);

                    WindowEvent::MouseMove { target }
                }
                glfw::WindowEvent::MouseButton(_button, action, _modifiers) => match action {
                    glfw::Action::Press => WindowEvent::MouseDown,
                    glfw::Action::Release => WindowEvent::MouseUp,
                    _ => unreachable!("mouse should not repeat"),
                },
                //glutin::WindowEvent::ReceivedCharacter(ch) => WindowEvent::KeyPress(ch as u16),
                //glutin::WindowEvent::CloseRequested => WindowEvent::Close,
                //glutin::WindowEvent::Resized(..) => WindowEvent::Resize,
                _ => WindowEvent::Unknown,
            }),
        }
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
