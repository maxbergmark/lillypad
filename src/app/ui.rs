#[allow(clippy::wildcard_imports)]
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

use crate::{
    app::{graph::Graph, provide_state, AppState},
    sensor::model::{SensorData, SensorType},
    server::TimeSpan,
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    provide_state();

    view! {
        <Stylesheet id="leptos" href="/pkg/leptos_start.css" />
        <Title text="Lilly's Environment" />
        <Router>
            <main class="bg-black">
                <Routes fallback=|| { view! { <NotFound /> }.into_view() }>
                    <Route
                        path=StaticSegment("/")
                        view=|| {
                            view! { <DataView /> }
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
fn DataView() -> impl IntoView {
    view! {
        <div class="bg-black h-full min-h-screen flex flex-col p-6 gap-8 select-none">
            <TimeSpanSelector />
            <DataBoxes />
        </div>
    }
}

#[component]
fn TimeSpanSelector() -> impl IntoView {
    #[allow(clippy::expect_used)]
    let app_state = use_context::<RwSignal<AppState>>().expect("No app state found");
    view! {
        <div
            class="btn-primary w-full h-20 p-2 pl-2 flex leading-none items-center space-x-2 bg-gradient-to-br from-blue-500 to-blue-800 rounded-xl"
            on:click=move |_| {
                app_state.update(AppState::next_time_span);
            }
        >
            <div class="max-w-full h-full flex relative basis-full font-sans font-bold">
                <div class="w-1/2 inline-flex h-full items-center justify-center">
                    <p
                        class:text-blue-300=move || app_state().time_span == TimeSpan::Day
                        class:text-white=move || app_state().time_span == TimeSpan::Hour
                        class="z-10 transition-all"
                    >
                        Past hour
                    </p>
                </div>
                <div class="w-1/2 inline-flex h-full items-center justify-center">
                    <p
                        class:text-blue-300=move || app_state().time_span == TimeSpan::Hour
                        class:text-white=move || app_state().time_span == TimeSpan::Day
                        class="z-10 transition-all"
                    >
                        Past day
                    </p>
                </div>
                <div
                    class:translate-x-full=move || app_state().time_span == TimeSpan::Day
                    class="w-1/2 h-full bg-blue-400 opacity-50 absolute transition-all rounded-xl fade-dark"
                />
            </div>
        </div>
    }
}

#[component]
fn DataBoxes() -> impl IntoView {
    view! {
        <div class="bg-black h-full min-h-screen flex flex-col gap-4">
            <DataBox sensor_type=SensorType::Temperature />
            <DataBox sensor_type=SensorType::Humidity />
            <DataBox sensor_type=SensorType::Barometric />
        </div>
    }
}

#[component]
fn DataBox(sensor_type: SensorType) -> impl IntoView {
    view! {
        <div class="grid basis-1/3 min-h-56 bg-gradient-to-br from-blue-500 to-blue-800 rounded-xl">
            <div class="col-start-1 row-start-1 w-full h-full flex items-center grow z-10">
                <Icon sensor_type />
                <DataValue sensor_type />
            </div>
            <div class="col-start-1 row-start-1 z-0 overflow-hidden">
                <Graph sensor_type />
            </div>
        </div>
    }
}

#[component]
fn DataValue(sensor_type: SensorType) -> impl IntoView {
    #[allow(clippy::expect_used)]
    let s = use_context::<Resource<Result<SensorData, ServerFnError>>>()
        .expect("No server state found");
    let value = move || {
        s.get()
            .and_then(std::result::Result::ok)
            .map_or_else(|| " ".to_string(), |s| sensor_type.format_data(&s))
    };
    view! {
        <Transition fallback=|| {}>
            <span class="text-white font-mono m-auto text-4xl md:text-8xl">{value}</span>
        </Transition>
    }
}

#[component]
fn Icon(sensor_type: SensorType) -> impl IntoView {
    let filename = match sensor_type {
        SensorType::Temperature => "/assets/temperature.png",
        SensorType::Humidity => "/assets/humidity.png",
        SensorType::Barometric => "/assets/pressure.png",
    };

    view! {
        <img
            src=filename
            alt=filename
            width=256
            height=256
            class="opacity-30 p-6 w-32 h-32 md:w-48 md:h-48"
        />
    }
}
