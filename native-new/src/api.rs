//! high-level API
//!
//! note that it is mostly write-only because it is intentionally
//! designed to be used with some client-side stateful wrapper/reconciler


/// Represents currently running application
///
/// you need app to create windows, access them & handle their events
///
/// in future it might provide some app-related things (notifications, icon highlighting, ...)
pub trait App<W: Window> {
    fn get_next_event(&mut self) -> Option<AppEvent>;

    fn create_window(&mut self) -> WindowId;
    fn get_window(&mut self, id: WindowId) -> &mut W;
    fn destroy_window(&mut self, id: WindowId);
}

pub enum AppEvent {
  WindowEvent {
    window: WindowId,
    event: WindowEvent
  }
}

pub enum WindowEvent {
  Close,
  Resize,
  Click
}

/// Represents a window, including all of its UI contents (scene)
///
/// any changes to the scene have to be made through facade/mediator which is provided to an
/// update_scene() call
///
/// technically, it doesn't even have to be a real window (embedded, mobile)
pub trait Window {
    fn update_scene<F>(&mut self, update_fn: F) where F: FnMut(&mut SceneUpdateContext);

    // platform-specific (and optional)
    //fn set_size(&mut self, _width: u32, _height: u32) {}
    //fn set_title(&mut self, _title: &str) {}
    fn show(&mut self) {}
    fn hide(&mut self) {}
}

/// Facade/mediator to an executed scene update
///
/// theoretically we could represent it as a message but we would need to return SurfaceId somehow
/// and it just doesn't feel right (for some reason)
pub trait SceneUpdateContext {
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
pub type WindowId = glutin::WindowId;
