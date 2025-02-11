pub mod ui;

use leptos::prelude::*;

use crate::{sensor::model::SensorState, server::get_cached_sensor_state};

fn provide_state() {
    let sensor_state = SensorState::default();
    let state = RwSignal::new(sensor_state);
    provide_context(state);

    let trigger = setup_trigger();
    let async_data = Resource::new(trigger, |_| get_cached_sensor_state());

    provide_context(async_data);
}

fn setup_trigger() -> ReadSignal<i32> {
    let (trigger, set_trigger) = signal(0_i32);
    if cfg!(target_family = "wasm") {
        set_interval(
            move || set_trigger.update(|c| *c += 1),
            std::time::Duration::from_secs(5),
        );
    }
    trigger
}
