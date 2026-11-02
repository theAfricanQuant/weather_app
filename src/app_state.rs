use crate::models::{WeatherQueryRequest, WeatherQueryResult};

#[derive(Debug)]
pub struct AppState {
    pub input_location: String,
    pub loading: bool,
    pub request: Option<WeatherQueryRequest>,
    pub latest_result: Option<WeatherQueryResult>,
    pub status: String,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            input_location: String::from("Lagos"),
            loading: false,
            request: None,
            latest_result: None,
            status: String::from("Ready"),
        }
    }
}
