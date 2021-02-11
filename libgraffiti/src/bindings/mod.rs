// bindings for deno & nodejs
// - thread-local storage, shared fns (this file)
// - submodules define macros and then include!("api.rs")


use crate::util::SlotMap;
use crate::{App, Window};
use std::cell::RefCell;
use std::rc::Rc;

//static VIEWPORTS: Lazy<Mutex<SlotMap<WindowId, Viewport>>> = lazy!(|| Mutex::new(SlotMap::new()));

thread_local! {
    static CTX: Rc<RefCell<Ctx>> = Default::default();
}

macro_rules! ctx {
    () => {
        super::CTX.with(|ctx| ctx.clone()).borrow_mut()
    };
}

type WindowId = u32;

#[derive(Default)]
struct Ctx {
    app: Option<App>,
    windows: SlotMap<WindowId, Window>,
}

impl Ctx {
    fn init_app(&mut self) {
        self.app = Some(unsafe { App::init() })
    }

    fn create_window(&mut self, title: &str, width: i32, height: i32) -> WindowId {
        let mut window = self.app.as_mut().expect("no app").create_window(&title, width, height);
        //let viewport = window.create_viewport();

        let id = self.windows.insert(window);

        //VIEWPORTS.lock().unwrap().put(id, viewport);

        id
    }

    fn tick(&mut self) {
        for (id, win) in self.windows.iter_mut() {
            if let Some(e) = win.take_event() {
                println!("TODO: {:?}", e);
            }

            //let viewport = &mut VIEWPORTS.lock().unwrap()[id];

            //viewport.update();
            //viewport.render();

            win.swap_buffers();
        }

        self.app.as_mut().expect("no app").wait_events_timeout(0.1);
    }
}

mod nodejs;
mod deno;
