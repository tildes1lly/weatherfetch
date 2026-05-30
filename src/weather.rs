use reqwest;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Current {
    #[serde(rename = "temperature_2m")]
    pub temperature: f64,
    #[serde(rename = "wind_speed_10m")]
    pub wind_speed: f64,
    #[serde(rename = "wind_direction_10m")]
    pub wind_direction: f64,
    #[serde(rename = "apparent_temperature")]
    pub feels_like: f64,
    #[serde(rename = "cloud_cover")]
    pub sky_condition_num: f64,
    #[serde(rename = "relative_humidity_2m")]
    pub humidity: f64,
}

#[derive(Deserialize)]
pub struct Daily {
    #[serde(rename = "sunrise")]
    pub sunrise: Vec<String>,
    #[serde(rename = "sunset")]
    pub sunset: Vec<String>,
    #[serde(rename = "uv_index_max")]
    pub uv_index: Vec<f64>,
    #[serde(rename = "temperature_2m_max")]
    pub temperature_max: Vec<f64>,
    #[serde(rename = "temperature_2m_min")]
    pub temperature_min: Vec<f64>,
    #[serde(rename = "precipitation_probability_max")]
    pub precipitation_probability: Vec<f64>,
}

#[derive(Deserialize)]
pub struct Hourly {
    #[serde(rename = "visibility")]
    pub visibility: Vec<f64>,
    #[serde(rename = "dew_point_2m")]
    pub dew_point: Vec<f64>,
}

#[derive(Deserialize)]
pub struct OpenMeteoResponse {
    pub current: Current,
    pub daily: Daily,
    pub hourly: Hourly,
}

#[derive(Deserialize)]
pub struct AirQuality {
    #[serde(rename = "current")]
    pub current: AirQualityCurrent,
}

#[derive(Deserialize)]
pub struct AirQualityCurrent {
    #[serde(rename = "us_aqi")]
    pub aqi: f64,
}

#[derive(Deserialize)]
pub struct DailyForecast {
    #[serde(rename = "sunrise")]
    pub sunrise: Vec<String>,
    #[serde(rename = "sunset")]
    pub sunset: Vec<String>,
    #[serde(rename = "precipitation_probability_max")]
    pub precipitation_probability: Vec<f64>,
    #[serde(rename = "temperature_2m_max")]
    pub temperature_max: Vec<f64>,
    #[serde(rename = "temperature_2m_min")]
    pub temperature_min: Vec<f64>,
    #[serde(rename = "cloud_cover_mean")]
    pub cloud_cover_mean: Vec<f64>,
}

#[derive(Deserialize)]
pub struct Forecast {
    pub daily: DailyForecast,
}

#[derive(Deserialize)]
pub struct WeatherCode {
    #[serde(rename = "weather_code")]
    pub code: i32,
}

#[derive(Deserialize)]
pub struct WeatherCodeResponse {
    pub current: WeatherCode,
}

pub fn fetch(lat: f64, lon: f64) -> Result<OpenMeteoResponse, reqwest::Error> {
    let url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&daily=sunrise,sunset,uv_index_max,temperature_2m_max,temperature_2m_min,precipitation_probability_max&hourly=visibility,dew_point_2m&current=wind_speed_10m,wind_direction_10m,surface_pressure,precipitation,temperature_2m,relative_humidity_2m,apparent_temperature,cloud_cover&timezone=auto&forecast_days=1",
        lat, lon
    );
    let response = reqwest::blocking::get(&url)?.json::<OpenMeteoResponse>()?;
    Ok(response)
}

pub fn fetch_air_quality(lat: f64, lon: f64) -> Result<AirQuality, reqwest::Error> {
    let url = format!(
        "https://air-quality-api.open-meteo.com/v1/air-quality?latitude={}&longitude={}&current=us_aqi&timezone=auto&forecast_days=1",
        lat, lon
    );
    let response = reqwest::blocking::get(&url)?.json::<AirQuality>()?;
    Ok(response)
}

pub fn fetch_forecast(lat: f64, lon: f64) -> Result<Forecast, reqwest::Error> {
    let url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&daily=temperature_2m_max,temperature_2m_min,sunrise,sunset,precipitation_probability_max,weather_code,cloud_cover_mean&timezone=auto",
        lat, lon
    );
    let response = reqwest::blocking::get(&url)?.json::<Forecast>()?;
    Ok(response)
}

pub fn fetch_weather_code(lat: f64, lon: f64) -> Result<WeatherCodeResponse, reqwest::Error> {
    let url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current=weather_code&timezone=auto&forecast_days=1",
        lat, lon
    );
    let response = reqwest::blocking::get(&url)?.json::<WeatherCodeResponse>()?;
    Ok(response)
}