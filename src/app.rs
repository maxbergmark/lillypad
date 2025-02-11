pub mod ui;

use leptos::{
    create_resource, create_rw_signal, create_signal, provide_context, set_interval, ReadSignal,
    SignalUpdate,
};

use crate::{sensor::model::SensorState, server::get_cached_sensor_state};

fn provide_state() {
    let sensor_state = SensorState::default();
    let state = create_rw_signal(sensor_state);
    provide_context(state);

    let trigger = setup_trigger();
    let async_data = create_resource(trigger, |_| get_cached_sensor_state());

    provide_context(async_data);
}

fn setup_trigger() -> ReadSignal<i32> {
    let (trigger, set_trigger) = create_signal(0_i32);
    if cfg!(target_family = "wasm") {
        set_interval(
            move || set_trigger.update(|c| *c += 1),
            std::time::Duration::from_secs(5),
        );
    }
    trigger
}
