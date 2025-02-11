
#[cfg(feature = "ssr")]
use std::{sync::{Arc, Mutex}, time::Instant};

#[cfg(feature = "ssr")]
use leptos::prelude::use_context;
use leptos::{prelude::ServerFnError, server};

#[cfg(feature = "ssr")]
use crate::sensor::{get_barometric, get_humidity, get_temperature, model::SensorState};
use crate::sensor::model::SensorData;


#[server]
#[allow(clippy::future_not_send)]
pub async fn get_cached_sensor_state() -> Result<SensorData, ServerFnError> {
    let app_state = use_context::<leptos_actix::Request>()
        .and_then(|r| r.app_data::<actix_web::web::Data<Arc<Mutex<SensorState>>>>().cloned());

    if let Some(app_state) = app_state {
        #[allow(clippy::use_debug)]
        app_state.lock()
            .map(|value| {
                println!("cached value: {:?}", value.data);
                value.data.clone()
            })
            .map_err(|e| ServerFnError::ServerError(e.to_string()))
    } else {
        println!("fallback");
        get_sensor_state().await
    }
}

#[server]
pub async fn get_sensor_state() -> Result<SensorData, ServerFnError> {
    // use crate::sensor::{get_barometric, get_humidity, get_temperature};

    let temperature = get_temperature().await?;
    let humidity = get_humidity().await?;
    let barometric = get_barometric().await?;
    println!("t: {}, h: {}, b: {}", temperature.value, humidity.value, barometric.value);

    Ok(SensorData {
        temperature: temperature.value,
        humidity: humidity.value,
        barometric: barometric.value,
    })
}

#[cfg(feature = "ssr")]
pub async fn update_sensor_state(app_state: Arc<Mutex<SensorState>>) -> Result<(), ServerFnError> {
    let response = get_sensor_state().await?;
    #[allow(clippy::use_debug)]
    println!("updating sensor state: {response:?}");

    let mut value = app_state.lock()?;
    value.data = response.clone();
    value.historical_data.push(response);
    value
        .time_data
        .push(Instant::now().elapsed().as_secs() as i32);
    drop(value);
    Ok(())
}
