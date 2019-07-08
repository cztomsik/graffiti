use crate::generated::{Event, WindowEvent, WindowId, UpdateSceneMsg};
use crate::render::WebrenderRenderer;
use crate::window::Window;
use crate::layout::YogaLayout;
use gleam::gl::GlFns;
use glfw::{Context, Glfw};
use std::collections::BTreeMap;
use std::sync::mpsc::Receiver;
use crate::text::SimpleTextLayout;

pub struct TheApp {
    glfw: Glfw,
    windows: BTreeMap<WindowId, (Window, glfw::Window, Receiver<(f64, glfw::WindowEvent)>)>,
    next_window_id: WindowId,
}

impl TheApp {
    pub fn init() -> Self {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).expect("could not init GLFW");

        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 2));
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));

        TheApp {
            glfw,
            windows: BTreeMap::new(),
            next_window_id: 1,
        }
    }
}

impl TheApp {
    pub fn get_events(&mut self, poll: bool) -> Vec<Event> {
        if poll {
            self.glfw.poll_events()
        } else {
            // wait a bit otherwise (save battery)
            self.glfw.wait_events_timeout(0.1);
        }

        // go through all windows, handle their events, collect all the resulting events and wrap them along with respective window_id
        self.windows
            .iter_mut()
            .flat_map(|(id, (window, glfw_window, events))| {
                glfw_window.make_current();
                let res = glfw::flush_messages(events)
                    .filter_map(move |(_, e)| Self::handle_window_event(window, glfw_window, e))
                    .map(move |e| Event::WindowEvent {
                        window: *id,
                        event: e,
                    });

                res
            })
            .collect()
    }

    fn handle_window_event(window: &mut Window, glfw_window: &mut glfw::Window, event: glfw::WindowEvent) -> Option<WindowEvent> {
        // TODO: we don't need Option currently so maybe we can remove it in the future
        match event {
            event => Some(match event {
                glfw::WindowEvent::CursorPos(x, y) => window.mouse_move((x as f32, y as f32)),
                glfw::WindowEvent::Scroll(delta_x, delta_y) => {
                    let res = window.scroll((delta_x as f32, delta_y as f32));
                    glfw_window.swap_buffers();
                    res
                }
                glfw::WindowEvent::MouseButton(_button, action, _modifiers) => match action {
                    glfw::Action::Press => window.mouse_down(),
                    glfw::Action::Release => window.mouse_up(),
                    _ => unreachable!("mouse should not repeat"),
                },
                //glutin::WindowEvent::ReceivedCharacter(ch) => WindowEvent::KeyPress(ch as u16),
                //glutin::WindowEvent::CloseRequested => WindowEvent::Close,
                glfw::WindowEvent::FramebufferSize(_, _) => {
                    //self.update_sizes();
                    WindowEvent::Resize
                }
                glfw::WindowEvent::Close => WindowEvent::Close,
                // TODO: repeat works for some keys but for some it doesn't
                // not sure if it's specific for mac (special chars overlay)
                glfw::WindowEvent::Key(_key, scancode, action, _modifiers) => match action {
                    glfw::Action::Release => WindowEvent::KeyUp(scancode as u16),
                    _ => WindowEvent::KeyDown(scancode as u16),
                },
                glfw::WindowEvent::Char(ch) => WindowEvent::KeyPress(ch as u16),
                _ => WindowEvent::Unknown,
            }),
        }
    }

    pub fn create_window(&mut self) -> WindowId {
        let (width, height) = (1024, 768);

        let (mut glfw_window, events) = self
            .glfw
            .create_window(width, height, "stain", glfw::WindowMode::Windowed)
            .expect("couldnt create GLFW window");

        glfw_window.make_current();
        glfw_window.set_all_polling(true);

        let id = self.next_window_id;
        let gl = unsafe { GlFns::load_with(|addr| glfw_window.get_proc_address(addr)) };
        // TODO: dpi
        let renderer = Box::new(WebrenderRenderer::new(gl, (width as i32, height as i32)));
        let layout = Box::new(YogaLayout::new((width as f32, height as f32)));
        let text_layout = Box::new(SimpleTextLayout::new());
        let window = Window::new(renderer, layout, text_layout);

        self.windows.insert(id, (window, glfw_window, events));

        self.next_window_id = self.next_window_id + 1;

        // vsync off (for now)
        //self.glfw.set_swap_interval(glfw::SwapInterval::None);

        id
    }

    pub fn update_window_scene(&mut self, id: WindowId, msgs: &[UpdateSceneMsg]) {
        let (window, glfw_window, _) = &mut self.windows.get_mut(&id).expect("window not found");

        window.update_scene(msgs);
        glfw_window.swap_buffers();
    }

    pub fn destroy_window(&mut self, id: WindowId) {
        self.windows.remove(&id);
    }
}
