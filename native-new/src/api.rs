//! high-level API
//!
//! note that it is mostly write-only because it is intentionally
//! designed to be used with some client-side stateful wrapper/reconciler

/// Represents currently running application
///
/// you need app to create windows, access them & handle their events
///
/// in future it might provide some app-related things (notifications, icon highlighting, ...)
pub trait App {
    fn get_next_event(&mut self) -> Option<Event>;

    fn create_window(&mut self) -> WindowId;
    fn get_window_mut(&mut self, id: WindowId) -> &mut Window;
    fn destroy_window(&mut self, id: WindowId);
}

pub use crate::generated::{Event, WindowEvent};

/// Represents a window, including all of its UI contents (scene)
///
/// technically, it doesn't have to be real window (embedded, mobile)
pub trait Window {
    fn scene_mut(&mut self) -> &mut Scene;
    fn render(&mut self);

    // platform-specific (and optional)
    //fn set_size(&mut self, _width: u32, _height: u32) {}
    //fn set_title(&mut self, _title: &str) {}
    fn show(&mut self) {}
    fn hide(&mut self) {}
}

/// Scene holds tree of surfaces, layout and some other related internal state
///
/// Surface is a "node" in the UI tree, it's similar to HTML element but:
/// - there's only one type
/// - text is just another property (there are no text "nodes")
///
/// it might be tempting to separate some `Surface` trait but:
/// - often after setting something, we need to set some internal state elsewhere
///   for example `set_size` makes the node and all of the parents dirty
/// - we went with the `struct of arrays` approach and pretending otherwise would
///   be both challenging and very confusing (so this is rather infancy but simple)
pub trait Scene {
    // structure
    fn create_surface(&mut self) -> SurfaceId;
    fn append_child(&mut self, parent: SurfaceId, child: SurfaceId);
    fn insert_before(&mut self, parent: SurfaceId, child: SurfaceId, before: SurfaceId);
    fn remove_child(&mut self, parent: SurfaceId, child: SurfaceId);
    fn children(&self, surface: SurfaceId) -> &[SurfaceId];

    // layout props
    fn set_size(&mut self, surface: SurfaceId, size: Size);
    fn set_flex(&mut self, surface: SurfaceId, flex: Flex);
    fn set_flow(&mut self, surface: SurfaceId, flow: Flow);
    fn set_padding(&mut self, surface: SurfaceId, padding: Rect);
    fn set_margin(&mut self, surface: SurfaceId, margin: Rect);

    // layout info
    fn computed_layout(&self, surface: SurfaceId) -> &ComputedLayout;

    // layout/visual
    fn border_radius(&self, surface: SurfaceId) -> Option<&BorderRadius>;
    fn set_border_radius(&mut self, surface: SurfaceId, border_radius: Option<BorderRadius>);

    // visual props
    fn box_shadow(&self, surface: SurfaceId) -> Option<&BoxShadow>;
    fn set_box_shadow(&mut self, surface: SurfaceId, box_shadow: Option<BoxShadow>);
    fn background_color(&self, surface: SurfaceId) -> Option<&Color>;
    fn set_background_color(&mut self, surface: SurfaceId, color: Option<Color>);
    fn image(&self, surface: SurfaceId) -> Option<&Image>;
    fn set_image(&mut self, surface: SurfaceId, image: Option<Image>);
    fn text(&self, surface: SurfaceId) -> Option<&Text>;
    fn set_text(&mut self, surface: SurfaceId, text: Option<Text>);
    fn border(&self, surface: SurfaceId) -> Option<&Border>;
    fn set_border(&mut self, surface: SurfaceId, border: Option<Border>);
}

// TODO: should be in codegen (onLayout is going to receive this)
pub type ComputedLayout = (f32, f32, f32, f32);

// re-export some value objects
pub use crate::generated::{
    Border, BorderRadius, BorderSide, BorderStyle, BoxShadow, Color, Dimension, Flex, Flow, Image,
    Rect, Size, SurfaceId, Text, TextAlign, WindowId, FlexAlign, FlexDirection, FlexWrap, JustifyContent
};
