mod app_state;
mod models;

use app_state::AppState;
use models::WeatherQueryResult;

fn main() {
    let mut state = AppState::new();
    println!("Starting state:\n{state:#?}");

    state.latest_result = Some(WeatherQueryResult {
        location: String::from("Lagos, Nigeria"),
        temperature_c: Some(31.0),
    });
    state.status = String::from("Weather loaded");

    println!("Updated state:\n{state:#?}");
}
