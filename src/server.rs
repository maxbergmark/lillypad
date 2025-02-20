
#[cfg(feature = "ssr")]
use std::sync::{Arc, Mutex};
#[cfg(feature = "ssr")]
use chrono::Utc;
#[cfg(feature = "ssr")]
use leptos::prelude::use_context;

use leptos::{prelude::ServerFnError, server};
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use crate::sensor::{get_barometric, get_humidity, get_temperature, model::SensorState};
use crate::sensor::model::SensorData;

#[cfg(feature = "ssr")]
mod util;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimeSpan {
    Hour,
    Day,
}


#[server]
#[allow(clippy::future_not_send)]
pub async fn get_cached_sensor_state() -> Result<SensorData, ServerFnError> {
    let app_state = use_context::<leptos_actix::Request>()
        .and_then(|r| r.app_data::<actix_web::web::Data<Arc<Mutex<SensorState>>>>().cloned());

    if let Some(app_state) = app_state {
        #[allow(clippy::use_debug)]
        app_state.lock()
            .map(|value| {
                value.data.clone()
            })
            .map_err(|e| ServerFnError::ServerError(e.to_string()))
    } else {
        println!("fallback");
        get_sensor_state().await
    }
}

#[server]
#[allow(clippy::future_not_send)]
#[allow(clippy::use_debug)]
pub async fn get_cached_historical_data(time_span: TimeSpan) -> Result<SensorState, ServerFnError> {
    let app_state = use_context::<leptos_actix::Request>()
        .and_then(|r| r.app_data::<actix_web::web::Data<Arc<Mutex<SensorState>>>>().cloned());

    app_state.map_or_else(
        || Err(ServerFnError::ServerError("No app state found".to_string())),
        |app_state| app_state.lock()
            .map(|value| f(&value, time_span))
            .map_err(|e| ServerFnError::ServerError(e.to_string()))
    )
}

fn f(value: &SensorState, time_span: TimeSpan) -> SensorState {
    let historical_data = util::filter_data(&value.historical_data, time_span);
    SensorState {
        data: value.data.clone(),
        historical_data,
    }
}

#[server]
pub async fn get_sensor_state() -> Result<SensorData, ServerFnError> {
    let temperature = get_temperature().await?;
    let humidity = get_humidity().await?;
    let barometric = get_barometric().await?;

    Ok(SensorData {
        temperature: temperature.value,
        humidity: humidity.value,
        barometric: barometric.value,
        time: Utc::now().timestamp() as i32,
    })
}

#[cfg(feature = "ssr")]
#[allow(clippy::use_debug)]
pub async fn update_sensor_state(app_state: Arc<Mutex<SensorState>>) -> Result<(), ServerFnError> {
    let response = get_sensor_state().await?;
    println!("updating sensor state: {response:?}");

    let mut value = app_state.lock()?;
    value.data = response.clone();
    value.historical_data.push(response);
    drop(value);
    Ok(())
}
