use crate::sensor::model::SensorData;

use super::TimeSpan;

pub fn filter_data(sensor_state: &[SensorData], time_span: TimeSpan) -> Vec<SensorData> {
    let now = chrono::Utc::now().timestamp();
    let start = match time_span {
        TimeSpan::Hour => now - 3600,
        TimeSpan::Day => now - 86400,
    };
    (0..100)
        .map(|i| f64::from(i) / 99.0)
        .map(|f| start + (f * (now - start) as f64) as i64)
        .filter_map(|t| get_reading_at_time(sensor_state, t as i32))
        .collect()
}

fn get_reading_at_time(sensor_state: &[SensorData], time: i32) -> Option<SensorData> {
    let idx = sensor_state.partition_point(|d| d.time <= time);
    match idx {
        0 => None,
        // 0 => Some(sensor_state[0].clone()),
        _ if idx == sensor_state.len() => Some(sensor_state[idx - 1].clone()),
        _ => Some(weighted_average(
            &sensor_state[idx - 1],
            &sensor_state[idx],
            time,
        )),
    }
}

fn weighted_average(a: &SensorData, b: &SensorData, time: i32) -> SensorData {
    let delta = b.time - a.time;
    let ratio = (time - a.time) as f32 / delta as f32;
    SensorData {
        temperature: (b.temperature - a.temperature).mul_add(ratio, a.temperature),
        humidity: (b.humidity - a.humidity).mul_add(ratio, a.humidity),
        barometric: (b.barometric - a.barometric).mul_add(ratio, a.barometric),
        time,
    }
}
