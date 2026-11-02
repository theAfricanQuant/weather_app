fn parse_coordinate(
    raw: &str,
    label: &str,
    min: f64,
    max: f64,
) -> Result<f64, String> {
    let value = raw
        .trim()
        .parse::<f64>()
        .map_err(|_| format!("{label} must be a number"))?;

    if !(min..=max).contains(&value) {
        return Err(format!("{label} must be between {min} and {max}"));
    }

    Ok(value)
}

pub fn parse_latitude(raw: &str) -> Result<f64, String> {
    parse_coordinate(raw, "Latitude", -90.0, 90.0)
}

pub fn parse_longitude(raw: &str) -> Result<f64, String> {
    parse_coordinate(raw, "Longitude", -180.0, 180.0)
}
