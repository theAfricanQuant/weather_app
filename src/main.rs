mod app_state;
mod input;
mod models;

use app_state::AppState;
use models::{LocationQuery, WeatherQueryRequest};

fn main() {
    for raw in ["6.52", "west", "120.0"] {
        println!(
            "Latitude {:?} -> {:?}",
            raw,
            input::parse_latitude(raw),
        );
    }

    let latitude = input::parse_latitude("6.52");
    let longitude = input::parse_longitude("3.38");
    let mut state = AppState::new();

    match (latitude, longitude) {
        (Ok(latitude), Ok(longitude)) => {
            state.request = Some(WeatherQueryRequest {
                location: LocationQuery::Coordinates {
                    latitude,
                    longitude,
                },
                start_date: String::from("2026-10-21"),
                end_date: String::from("2026-10-22"),
            });
            state.status = String::from("Coordinates validated");
        }
        (Err(error), _) | (_, Err(error)) => {
            state.status = error;
        }
    }

    println!("Request state:\n{state:#?}");
}
