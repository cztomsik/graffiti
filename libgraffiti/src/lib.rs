#[macro_use]
mod util;

mod api;
mod app;
mod window;
mod viewport;
mod document;
mod css;
mod style;
mod selector;
mod layout;
mod gfx;

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

mod unstable {
    pub use crate::app::App;
    pub use crate::window::Window;
    pub use crate::document::Document;
}
