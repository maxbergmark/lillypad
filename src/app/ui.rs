#[allow(clippy::wildcard_imports)]
use leptos::*;
use leptos_image_optimizer::{provide_image_context, Image};
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{Route, Router, Routes};

use crate::{
    app::provide_state,
    sensor::model::{SensorData, SensorType},
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    provide_image_context();
    provide_state();

    view! {
        <Stylesheet id="leptos" href="/pkg/leptos_start.css" />
        <Title text="Lilly's Environment" />
        <Router fallback=|| { view! { <NotFound /> }.into_view() }>
            <main>
                <Routes>
                    <Route
                        path="/"
                        view=|| {
                            view! { <DataChart /> }
                        }
                    />
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn NotFound() -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! { <h1>"Not Found"</h1> }
}

#[component]
fn DataChart() -> impl IntoView {
    view! {
        <div class="bg-black h-full min-h-screen flex flex-col gap-1">
            <DataBox sensor_type=SensorType::Temperature />
            <DataBox sensor_type=SensorType::Humidity />
            <DataBox sensor_type=SensorType::Barometric />
        </div>
    }
}

#[component]
fn DataBox(sensor_type: SensorType) -> impl IntoView {
    view! {
        <div class="flex items-center grow basis-1/3 bg-gradient-to-tl from-blue-800 to-blue-500 m-6 rounded-xl">
            <Icon sensor_type />
            <DataValue sensor_type />
        </div>
    }
}

#[component]
fn DataValue(sensor_type: SensorType) -> impl IntoView {
    #[allow(clippy::expect_used)]
    let s = use_context::<Resource<i32, Result<SensorData, leptos::ServerFnError>>>()
        .expect("No server state found");
    let value = move || {
        s.get()
            .and_then(std::result::Result::ok)
            .map_or_else(|| " ".to_string(), |s| sensor_type.format_data(&s))
    };
    view! {
        <Transition fallback=|| {
            view! {}
        }>
            <span class="text-white font-mono m-auto text-4xl md:text-8xl">{value}</span>
        </Transition>
    }
}

#[component]
fn Icon(sensor_type: SensorType) -> impl IntoView {
    let filename = match sensor_type {
        SensorType::Temperature => "temperature.png",
        SensorType::Humidity => "humidity.png",
        SensorType::Barometric => "pressure.png",
    };

    view! {
        <Image
            alt=filename
            src=filename
            width=100
            height=100
            quality=100
            blur=false
            class="opacity-30 p-6"
        />
    }
}
