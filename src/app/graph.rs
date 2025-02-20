#[allow(clippy::wildcard_imports)]
use leptos::prelude::*;
#[allow(clippy::wildcard_imports)]
use leptos_chartistry::*;

use crate::sensor::model::{SensorData, SensorState, SensorType};

#[derive(Debug)]
pub struct Pair {
    x: f64,
    y: f64,
}

const fn get_data(d: &SensorData, sensor_type: SensorType) -> f64 {
    let v = match sensor_type {
        SensorType::Temperature => d.temperature,
        SensorType::Humidity => d.humidity,
        SensorType::Barometric => d.barometric / 100.0,
    };
    v as f64
}

fn into_series(data: &[SensorData], sensor_type: SensorType) -> Vec<Pair> {
    data.iter()
        .map(|d| Pair {
            x: f64::from(d.time),
            y: get_data(d, sensor_type),
        })
        .collect()
}

#[allow(clippy::missing_panics_doc)]
#[allow(clippy::use_debug)]
pub fn load_historical_signal(sensor_type: SensorType) -> Signal<Vec<Pair>> {
    println!("load historical");
    #[allow(clippy::expect_used)]
    let sensor_state = use_context::<Resource<Result<SensorState, ServerFnError>>>()
        .expect("No server state found");

    let (data_pairs_signal, set_data_pairs_signal) = signal(Vec::new());

    Effect::new(move |_| {
        if let Some(result) = sensor_state.get() {
            match result {
                Ok(s) => {
                    let transformed_data = into_series(&s.historical_data, sensor_type);
                    set_data_pairs_signal.set(transformed_data);
                }
                Err(_) => {
                    set_data_pairs_signal.set(Vec::new());
                }
            }
        }
    });

    data_pairs_signal.into()
}

const fn get_min_y(sensor_type: SensorType) -> f64 {
    match sensor_type {
        SensorType::Temperature => 17.0,
        SensorType::Humidity => 30.0,
        SensorType::Barometric => 1000.0,
    }
}

const fn get_max_y(sensor_type: SensorType) -> f64 {
    match sensor_type {
        SensorType::Temperature => 23.0,
        SensorType::Humidity => 70.0,
        SensorType::Barometric => 1050.0,
    }
}

#[component]
pub fn Graph(sensor_type: SensorType) -> impl IntoView {
    let series = Series::new(|data: &Pair| data.x)
        .line(
            Line::new(|data: &Pair| data.y)
                .with_name("y")
                .with_colour(Colour::from_rgb(59, 130, 246))
                .with_width(5.0)
                .with_interpolation(Interpolation::Linear),
        )
        .with_min_y(get_min_y(sensor_type))
        .with_max_y(get_max_y(sensor_type));
    view! {
        <div class="h-full">
            <Chart
                aspect_ratio=AspectRatio::from_env()
                series=series
                data=load_historical_signal(sensor_type)
                tooltip=Tooltip::left_cursor().show_x_ticks(false)
            />
        </div>
    }
}

// aspect_ratio=AspectRatio::from_outer_height(500.0, 1.0)
// AxisMarker::left_edge().into_inner(),
// AxisMarker::bottom_edge().into_inner(),
// top=RotatedLabel::middle("Hello, hydration!")
// XGridLine::default().into_inner(),
// left=TickLabels::aligned_floats()
// bottom=Legend::end()
