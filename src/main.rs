mod models;

use models::{LocationQuery, WeatherQueryRequest};

fn main() {
    let request = WeatherQueryRequest {
        location: LocationQuery::CityState {
            city: String::from("Lagos"),
            state: String::from("Lagos"),
        },
        start_date: String::from("2026-10-21"),
        end_date: String::from("2026-10-22"),
    };

    println!("{request:#?}");
}
