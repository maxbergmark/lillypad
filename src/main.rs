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

#![recursion_limit = "256"]

use cfg_if as _;
use console_error_panic_hook as _;
use leptos as _;
// use leptos_image_optimizer as _;
use leptos_meta as _;
use leptos_router as _;
use mio as _;
use serde as _;
use serde_json as _;
use wasm_bindgen as _;
use tracing as _;
use derive_more as _;
use leptos_chartistry as _;
use leptos_use as _;
use reqwest as _;
use chrono as _;

#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use std::sync::Arc;

    use actix_files::Files;
    use actix_web::{middleware::Logger, web, App, HttpServer};
    use leptos::{config::get_configuration, prelude::{provide_context, AutoReload, GlobalAttributes, HydrationScripts}};
    // use actix_web::*;
    #[allow(clippy::wildcard_imports)]
    use leptos::*;
    use leptos::prelude::ElementChild;
    use leptos_actix::{generate_route_list, LeptosRoutes};
    use leptos_meta::MetaTags;
    // use leptos_image_optimizer::{actix_handler, cache_app_images};
    // use leptos_image_optimizer::*;
    use lillypad::{app::ui::App, server::get_sensor_state};

    // let resp = lillypad::sensors::get_humidity().await;
    // println!("{:?}", resp);

    #[allow(clippy::expect_used)]
    let sensor_state = get_sensor_state().await.expect("Failed to get sensor state");
    let app_state = Arc::new(Mutex::new(SensorState::new(sensor_state)));
    setup_loop(app_state.clone());
    let shared_state = web::Data::new(app_state);
    provide_context(shared_state.clone());

    #[allow(clippy::expect_used)]
    let conf = get_configuration(None).expect("Failed to get configuration");
    let addr = conf.leptos_options.site_addr;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);
    println!("listening on http://{}", &addr);
    let root = conf.leptos_options.site_root.clone();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(move || {
        let leptos_options = &conf.leptos_options;
        let site_root = &leptos_options.site_root;

        #[allow(clippy::literal_string_with_formatting_args)]
        App::new()
            .app_data(web::Data::new(leptos_options.to_owned()))
            .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
            // .service(Files::new("/assets", "."))
            .service(actix_files::Files::new("/assets", &*root).show_files_listing())
            .leptos_routes(routes.clone(), {
                let leptos_options = leptos_options.clone();
                move || {
                    view! {
                        <!DOCTYPE html> 
                        <html lang="en">
                            <head>
                                <meta charset="utf-8" />
                                <meta
                                    name="viewport"
                                    content="width=device-width, initial-scale=1"
                                />
                                <AutoReload options=leptos_options.clone() />
                                <HydrationScripts options=leptos_options.clone() />
                                <MetaTags />
                            </head>
                            <body>
                                <App />
                            </body>
                        </html>
                    }
                }
            })
            .service(Files::new("/pkg", format!("{site_root}/pkg")))
            .app_data(shared_state.clone())
            .app_data(web::Data::new(leptos_options.to_owned()))
            // serve other assets from the `assets` directory
            // .route("/cache/image", web::get().to(actix_handler))
            // serve JS/WASM/CSS from `pkg`
            // serve the favicon from /favicon.ico
            .service(favicon)
            .wrap(Logger::default())

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
    use actix_web::rt::{spawn, time::interval};
    use lillypad::server::update_sensor_state;
    use std::time::Duration;

    spawn(async move {
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
    leptos_options: actix_web::web::Data<leptos::config::LeptosOptions>,
) -> actix_web::Result<actix_files::NamedFile> {

    let leptos_options = leptos_options.into_inner();
    let site_root = &leptos_options.site_root;
    Ok(actix_files::NamedFile::open(format!(
        "{site_root}/favicon.ico"
    ))?)
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
    // see optional feature `csr` instead
}
