pub mod model;
use model::{SensorResponse, SensorType};

use crate::{Error, Result};

pub async fn get_temperature() -> Result<SensorResponse> {
    get_sensor_response(SensorType::Temperature).await
}

pub async fn get_humidity() -> Result<SensorResponse> {
    get_sensor_response(SensorType::Humidity).await
}

pub async fn get_barometric() -> Result<SensorResponse> {
    get_sensor_response(SensorType::Barometric).await
}

async fn get_sensor_response(sensor_type: SensorType) -> Result<SensorResponse> {
    let url = format!("http://192.168.10.40/sensor?sensor_type={sensor_type}&index=0");
    reqwest::get(url)
        .await?
        .json::<SensorResponse>()
        .await
        .map_err(|e| Error::SensorError(e.to_string()))
}