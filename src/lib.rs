pub mod app;
pub mod error;
pub mod sensor;
pub mod server;

pub use error::{Error, Result};

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::ui::App;

    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
