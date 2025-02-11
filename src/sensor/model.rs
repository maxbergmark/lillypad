use std::{fmt::Display, time::Instant};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SensorState {
    pub data: SensorData,
    pub historical_data: Vec<SensorData>,
    pub time_data: Vec<i32>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SensorData {
    pub temperature: f32,
    pub humidity: f32,
    pub barometric: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SensorResponse {
    pub sensor_type: SensorType,
    pub index: i32,
    pub value: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum SensorType {
    Temperature,
    Humidity,
    Barometric,
}

impl SensorState {
    pub fn new(state: SensorData) -> Self {
        Self {
            data: state.clone(),
            historical_data: vec![state],
            time_data: vec![Instant::now().elapsed().as_secs() as i32],
        }
    }
}

impl SensorType {
    pub fn format(&self, value: f32) -> String {
        match self {
            Self::Temperature => format!("{value:.1}°C"),
            Self::Humidity => format!("{value:.1}%"),
            Self::Barometric => format!("{:.1}hPa", value / 100.0),
        }
    }

    pub fn format_data(&self, data: &SensorData) -> String {
        match self {
            Self::Temperature => self.format(data.temperature),
            Self::Humidity => self.format(data.humidity),
            Self::Barometric => self.format(data.barometric),
        }
    }
}

impl Display for SensorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Temperature => write!(f, "temperature"),
            Self::Humidity => write!(f, "humidity"),
            Self::Barometric => write!(f, "barometric"),
        }
    }
}
