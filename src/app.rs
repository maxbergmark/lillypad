mod graph;
pub mod ui;
use leptos::prelude::*;

use crate::{
    sensor::model::SensorState,
    server::{get_cached_historical_data, get_cached_sensor_state, TimeSpan},
};

#[derive(Debug, Clone)]
pub struct AppState {
    time_span: TimeSpan,
}

impl AppState {
    pub const fn next_time_span(&mut self) {
        self.time_span = match self.time_span {
            TimeSpan::Hour => TimeSpan::Day,
            TimeSpan::Day => TimeSpan::Hour,
        };
    }
}

fn provide_state() {
    let sensor_state = SensorState::default();
    let sensor_state_signal = RwSignal::new(sensor_state);
    provide_context(sensor_state_signal);

    let app_state = AppState {
        time_span: TimeSpan::Hour,
    };
    let app_state_signal = RwSignal::new(app_state);
    provide_context(app_state_signal);

    let trigger = setup_trigger(app_state_signal);
    let async_data = Resource::new(trigger, |_| get_cached_sensor_state());
    let historical_data = Resource::new(trigger, move |_| {
        get_cached_historical_data(app_state_signal.get().time_span)
    });

    provide_context(async_data);
    provide_context(historical_data);
}

fn setup_trigger(app_state_signal: RwSignal<AppState>) -> ReadSignal<i32> {
    let (trigger, set_trigger) = signal(0_i32);
    Effect::new(move |_| {
        app_state_signal.with(|_| set_trigger.update(|c| *c += 1));
    });
    if cfg!(target_family = "wasm") {
        set_interval(
            move || set_trigger.update(|c| *c += 1),
            std::time::Duration::from_secs(5),
        );
    }
    trigger
}
