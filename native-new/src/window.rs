use crate::api::{
    Border, BorderRadius, BoxShadow, Color, Dimension, Flex, Flow, Image, Rect, SceneUpdateContext,
    Size, SurfaceId, Text, Window, WindowEvent,
};
use crate::layout::{LayoutService, YogaLayoutService};
use crate::render::{RenderService, WebrenderRenderService};
use crate::scene::Scene;
use gleam::gl::GlFns;
use glutin::{ContextTrait, WindowId, WindowedContext};

pub struct GlutinWindow {
    glutin_context: WindowedContext,
    scene: Scene,
    layout_service: YogaLayoutService,
    render_service: WebrenderRenderService,
}

impl GlutinWindow {
    pub fn new(glutin_context: WindowedContext) -> Self {
        let gl =
            unsafe { GlFns::load_with(|addr| glutin_context.get_proc_address(addr) as *const _) };

        let mut window = GlutinWindow {
            glutin_context,
            scene: Scene::new(),
            layout_service: YogaLayoutService::new(),
            render_service: WebrenderRenderService::new(gl),
        };

        window.update_scene(|ctx| {
            ctx.create_surface();
        });

        window
    }

    pub fn id(&self) -> WindowId {
        self.glutin_context.id()
    }

    fn render(&mut self) {
        let surface = self.scene.get_surface_data(0);

        // TODO: set on resize
        let layout_size = self.render_service.layout_size;
        self.layout_service.set_size(
            0,
            Size(
                Dimension::Point(layout_size.width),
                Dimension::Point(layout_size.height),
            ),
        );

        let computed_layouts = self.layout_service.get_computed_layouts(&surface);

        self.render_service.render(&surface, computed_layouts);

        self.glutin_context.swap_buffers().ok();
    }

    // TODO
    pub fn translate_event(&self, event: glutin::WindowEvent) -> WindowEvent {
        match event {
            glutin::WindowEvent::CloseRequested => WindowEvent::Close,
            glutin::WindowEvent::Resized(..) => WindowEvent::Resize,
            _ => {
                // TODO: other events
                // unimplemented!()
                WindowEvent::Click
            }
        }
    }
}

impl Window for GlutinWindow {
    fn update_scene<F>(&mut self, mut update_fn: F)
    where
        F: FnMut(&mut SceneUpdateContext),
    {
        update_fn(self);
        self.render();
    }
}

// delegates to self.scene/layout_service with few special-cases where both have to be updated
impl SceneUpdateContext for GlutinWindow {
    fn create_surface(&mut self) -> SurfaceId {
        self.layout_service.alloc();
        self.scene.create_surface()
    }

    fn append_child(&mut self, parent: SurfaceId, child: SurfaceId) {
        self.scene.append_child(parent, child);
        self.layout_service.append_child(parent, child);
    }

    fn insert_before(&mut self, parent: SurfaceId, child: SurfaceId, before: SurfaceId) {
        let data = self.scene.get_surface_data(parent);
        let index = data
            .children()
            .position(|child| child.id() == before)
            .expect("not found");

        self.scene.insert_before(parent, child, before);
        self.layout_service.insert_at(parent, child, index as u32);
    }

    fn remove_child(&mut self, parent: SurfaceId, child: SurfaceId) {
        self.scene.remove_child(parent, child);
        self.layout_service.remove_child(parent, child);
    }

    fn set_size(&mut self, surface: SurfaceId, size: Size) {
        self.layout_service.set_size(surface, size);
    }

    fn set_flex(&mut self, surface: SurfaceId, flex: Flex) {
        self.layout_service.set_flex(surface, flex);
    }

    fn set_flow(&mut self, surface: SurfaceId, flow: Flow) {
        self.layout_service.set_flow(surface, flow);
    }

    fn set_padding(&mut self, surface: SurfaceId, padding: Rect) {
        self.layout_service.set_padding(surface, padding);
    }

    fn set_margin(&mut self, surface: SurfaceId, margin: Rect) {
        self.layout_service.set_margin(surface, margin);
    }

    fn set_border_radius(&mut self, surface: SurfaceId, border_radius: Option<BorderRadius>) {
        self.scene.set_border_radius(surface, border_radius);
    }

    fn set_box_shadow(&mut self, surface: SurfaceId, box_shadow: Option<BoxShadow>) {
        self.scene.set_box_shadow(surface, box_shadow);
    }

    fn set_background_color(&mut self, surface: SurfaceId, color: Option<Color>) {
        self.scene.set_background_color(surface, color);
    }

    fn set_image(&mut self, surface: SurfaceId, image: Option<Image>) {
        self.scene.set_image(surface, image);
    }

    fn set_text(&mut self, surface: SurfaceId, text: Option<Text>) {
        self.scene.set_text(surface, text.clone());
        self.layout_service.set_text(surface, text);
    }

    fn set_border(&mut self, surface: SurfaceId, border: Option<Border>) {
        self.scene.set_border(surface, border);
        // TODO: layout_service.set_border
    }
}
