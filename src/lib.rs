pub mod app;
pub mod error;
pub mod sensor;
pub mod server;
use cfg_if::cfg_if;

pub use error::{Error, Result};

cfg_if! {
if #[cfg(feature = "hydrate")] {

  use wasm_bindgen::prelude::wasm_bindgen;
  use leptos::mount::hydrate_body;

    #[wasm_bindgen]
    pub fn hydrate() {
      use app::ui::App;

      console_error_panic_hook::set_once();

      hydrate_body(App);
    }
}
}
