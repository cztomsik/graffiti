#[macro_use]
mod util;

mod api;
mod app;
mod css;
mod document;
mod gfx;
mod layout;
mod render;
mod viewport;
mod window;

// public but not yet semver stable
pub use api::{Api, WindowId};
pub use document::NodeId;

mod bindings {
    use crate::api::Api;
    use crate::app::App;
    use crate::util::Lazy;

    // shared for all bindings
    pub static API: Lazy<Api> = lazy!(|| Api::new(App::new()));

    #[cfg(feature = "deno")]
    mod deno;

    #[cfg(feature = "nodejs")]
    mod nodejs;

    //#[cfg(feature = "quickjs")]
    //mod quickjs;
}

pub mod unstable {
    pub use crate::app::App;
    pub use crate::document::Document;
    pub use crate::gfx::Surface;
    pub use crate::viewport::Viewport;
    pub use crate::window::Window;
}

}
