mod display;
mod args;
mod config;
mod location;
mod weather;

use chrono::prelude::*;
use reqwest;
use serde::Deserialize;

#[derive(Deserialize)]
struct CustomLocation {
    city: Option<String>,
    town: Option<String>,
    county: Option<String>,
    village: Option<String>,
    municipality: Option<String>,
    hamlet: Option<String>,
    quarter: Option<String>,

    state: Option<String>,
    region: Option<String>,
    province: Option<String>,
    country: Option<String>,
}

#[derive(Deserialize)]
struct NominatimResponse {
    address: CustomLocation,
}

fn main() {

    #[cfg(target_os = "windows")] // windows fix because windows sucks ass and everyone hates windows please switch to linux i never want to debug windows specific niche bugs again i hate you microsoft
    colored::control::set_virtual_terminal(true).ok();

    let args: Vec<String> = std::env::args().collect();
    let raw_config = args::parse(args);
    let config = raw_config.unwrap_or_else(|| config::gen_config());

    let lat;
    let lon;
    
    let mut ip_info: location::IPInfo = location::IPInfo { 
        latitude: 66.6, 
        longitude: 66.6,
        city: String::from("Custom"),
        region: String::from("Location") 
    };

    if let Some(custom_location) = &config.custom_location {
        lat = custom_location.lat;
        lon = custom_location.lon;
        let client = reqwest::blocking::Client::builder()
            .user_agent("weatherfetch/1.2.3")
            .build()
            .unwrap();
        let url: String = format!("https://nominatim.openstreetmap.org/reverse?lat={}&lon={}&format=json&accept-language=en", lat, lon);
        if let Ok(response) = client.get(&url).send().and_then(|r| r.json::<NominatimResponse>()) {
            let city = response.address.city
                .or(response.address.town)
                .or(response.address.village)
                .or(response.address.municipality)
                .or(response.address.county)
                .or(response.address.hamlet)
                .or(response.address.quarter)
                .unwrap_or(String::from("Unknown"));

            let region = response.address.state
                .or(response.address.region)
                .or(response.address.province)
                .or(response.address.country)
                .unwrap_or(String::from("Unknown"));
            ip_info = location::IPInfo {
                latitude: lat,
                longitude: lon,
                city: city,
                region: region,
            };
        }
    } else {
        ip_info = location::fetch_ip_info().expect("Failed to fetch IP info");
        lat = ip_info.latitude;
        lon = ip_info.longitude;
    }

    let open_meteo_response = weather::fetch(lat, lon).expect("Failed to fetch weather data");
    let air_quality_response = weather::fetch_air_quality(lat, lon).expect("Failed to fetch air quality data");
    let forecast_response = weather::fetch_forecast(lat, lon).expect("Failed to fetch forecast data");

    println!("{}", display::output(open_meteo_response, air_quality_response, forecast_response, Local::now(), ip_info, config));
}