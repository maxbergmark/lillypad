#![warn(
    // missing_docs,
    // unreachable_pub,
    keyword_idents,
    unexpected_cfgs,
    missing_copy_implementations,
    missing_debug_implementations,
    non_ascii_idents,
    noop_method_call,
    unused_crate_dependencies,
    unused_extern_crates,
    unused_import_braces,
    future_incompatible,
    nonstandard_style,
    bad_style,
    dead_code,
    improper_ctypes,
    non_shorthand_field_patterns,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    unconditional_recursion,
    unused,
    unused_allocation,
    unused_comparisons,
    unused_parens,
    while_true,
)]

use cfg_if as _;
use console_error_panic_hook as _;
use leptos as _;
use leptos_image_optimizer as _;
use leptos_meta as _;
use leptos_router as _;
use mio as _;
use serde as _;
use serde_json as _;
use wasm_bindgen as _;
use tracing as _;
use derive_more as _;
use leptos_chartistry as _;
use reqwest as _;

#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use std::sync::Arc;

    use actix_files::Files;
    use actix_web::{web, App, HttpServer};
    // use actix_web::*;
    #[allow(clippy::wildcard_imports)]
    use leptos::*;
    use leptos_actix::{generate_route_list, LeptosRoutes};
    use leptos_image_optimizer::{actix_handler, cache_app_images};
    // use leptos_image_optimizer::*;
    use lillypad::{app::ui::App, server::get_sensor_state};

    // let resp = lillypad::sensors::get_humidity().await;
    // println!("{:?}", resp);

    #[allow(clippy::expect_used)]
    let sensor_state = get_sensor_state().await.expect("Failed to get sensor state");
    let app_state = Arc::new(Mutex::new(SensorState::new(sensor_state)));
    setup_loop(app_state.clone());
    let shared_state = web::Data::new(app_state);

    #[allow(clippy::expect_used)]
    let conf = get_configuration(None).await.expect("Failed to get configuration");
    let addr = conf.leptos_options.site_addr;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);
    println!("listening on http://{}", &addr);
    let root = conf.leptos_options.site_root.clone();

    // run cache app images only in server

    #[allow(clippy::expect_used)]
    cache_app_images(root, || view! { <App /> }, 2, || (), || ())
        .await
        .expect("Failed to cache images");

    HttpServer::new(move || {
        let leptos_options = &conf.leptos_options;
        let site_root = &leptos_options.site_root;

        #[allow(clippy::literal_string_with_formatting_args)]
        App::new()
            .app_data(web::Data::new(leptos_options.to_owned()))
            .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
            .service(Files::new("/assets", site_root))
            .leptos_routes(leptos_options.to_owned(), routes.clone(), App)
            .service(Files::new("/pkg", format!("{site_root}/pkg")))
            .app_data(shared_state.clone())
            // serve other assets from the `assets` directory
            .route("/cache/image", web::get().to(actix_handler))
            // serve JS/WASM/CSS from `pkg`
            // serve the favicon from /favicon.ico
            .service(favicon)

        //.wrap(middleware::Compress::default())
    })
    .bind(&addr)?
    .run()
    .await
}

#[cfg(feature = "ssr")]
use std::sync::Arc;
#[cfg(feature = "ssr")]
use std::sync::Mutex;
#[cfg(feature = "ssr")]
use lillypad::sensor::model::SensorState;

#[cfg(feature = "ssr")]
fn setup_loop(app_state: Arc<Mutex<SensorState>>) {
    use actix_web::rt::time::interval;
    use leptos::spawn_local;
    use lillypad::server::update_sensor_state;
    use std::time::Duration;

    spawn_local(async move {
        let mut interval = interval(Duration::from_secs(5));
        loop {
            interval.tick().await;
            let _ = update_sensor_state(app_state.clone()).await
                .inspect_err(|e| println!("Error updating sensor state: {e}"));
        }
    });
}


#[cfg(feature = "ssr")]
#[actix_web::get("favicon.ico")]
async fn favicon(
    leptos_options: actix_web::web::Data<leptos::LeptosOptions>,
) -> actix_web::Result<actix_files::NamedFile> {
    let leptos_options = leptos_options.into_inner();
    let site_root = &leptos_options.site_root;
    Ok(actix_files::NamedFile::open(format!(
        "{site_root}/favicon.ico"
    ))?)
}

#[cfg(not(any(feature = "ssr", feature = "csr")))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
    // see optional feature `csr` instead
}

#[cfg(all(not(feature = "ssr"), feature = "csr"))]
pub fn main() {
    // a client-side main function is required for using `trunk serve`
    // prefer using `cargo leptos serve` instead
    // to run: `trunk serve --open --features csr`
    use leptos::*;
    use lillypad::app::*;
    use wasm_bindgen::prelude::wasm_bindgen;

    console_error_panic_hook::set_once();

    leptos::mount_to_body(move |cx| {
        // note: for testing it may be preferrable to replace this with a
        // more specific component, although leptos_router should still work
        view! { cx, <App /> }
    });
}
