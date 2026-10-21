#[derive(Debug)]
pub enum LocationQuery {
    CityState { city: String, state: String },
    Coordinates { latitude: f64, longitude: f64 },
}

#[derive(Debug)]
pub struct WeatherQueryRequest {
    pub location: LocationQuery,
    pub start_date: String,
    pub end_date: String,
}
