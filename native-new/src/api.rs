/// high-level interface/access-point to everything (what's important)
///
/// the point is that it doesn't matter how it's implemented inside,
/// if it's data-oriented or not, what technologies are used for layout, rendering, etc.
///
/// technically, it doesn't even have to be real window (embedded, mobile)
///
/// note that there is generally very limited amount of getters because it is intentionally
/// designed to be used with some client-side stateful wrapper/reconciler
pub trait Window {
    fn get_scene(&mut self) -> &mut SceneFacade;
    fn render(&mut self);

    // platform-specific (and optional)
    //fn set_size(&mut self, _width: u32, _height: u32) {}
    //fn set_title(&mut self, _title: &str) {}
    //fn show(&mut self) {}
    //fn hide(&mut self) {}
}

/// another facade to a tree of surfaces and their properties
/// it IS part of window's interface but it's separate so that it can freely evolve
/// without worrying about name collisions
pub trait SceneFacade {
    // structure
    fn create_surface(&mut self) -> SurfaceId;
    fn append_child(&mut self, parent: SurfaceId, child: SurfaceId);
    fn insert_before(&mut self, parent: SurfaceId, child: SurfaceId, before: SurfaceId);
    fn remove_child(&mut self, parent: SurfaceId, child: SurfaceId);

    // layout props
    fn set_size(&mut self, surface: SurfaceId, size: Size);
    fn set_flex(&mut self, surface: SurfaceId, flex: Flex);
    fn set_flow(&mut self, surface: SurfaceId, flow: Flow);
    fn set_padding(&mut self, surface: SurfaceId, padding: Rect);
    fn set_margin(&mut self, surface: SurfaceId, margin: Rect);

    // layout/visual
    fn set_border_radius(&mut self, surface: SurfaceId, border_radius: Option<BorderRadius>);

    // visual props
    fn set_box_shadow(&mut self, surface: SurfaceId, box_shadow: Option<BoxShadow>);
    fn set_background_color(&mut self, surface: SurfaceId, color: Option<Color>);
    fn set_image(&mut self, surface: SurfaceId, image: Option<Image>);
    fn set_text(&mut self, surface: SurfaceId, text: Option<Text>);
    fn set_border(&mut self, surface: SurfaceId, border: Option<Border>);
}

// re-export some value objects
pub use crate::generated::{
    Border, BorderRadius, BorderSide, BorderStyle, BoxShadow, Color, Dimension, Flex, Flow, Image,
    Rect, Size, SurfaceId, Text,
};
// TODO: gen
pub type WindowId = usize;

pub trait Application {
    fn create_window(&mut self) -> WindowId;
    fn get_window(&mut self, window_id: WindowId) -> &Window;
    fn destroy_window(&mut self, window: WindowId);
}

/*struct SomeAppWindowImpl {
    ui_tree: UiTree,
}*/
