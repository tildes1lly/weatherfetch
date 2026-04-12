mod display;
mod args;
mod config;
mod location;
mod weather;

use chrono::prelude::*;

fn main() {
    
    #[cfg(target_os = "windows")]
    colored::control::set_virtual_terminal(true).ok();
    let args: Vec<String> = std::env::args().collect();
    let raw_config = args::parse(args);
    let config = raw_config.unwrap_or_else(|| config::gen_config());

    let ip_info = location::fetch_ip_info().expect("Failed to fetch IP info");
    let lat = ip_info.latitude;
    let lon = ip_info.longitude;

    let open_meteo_response = weather::fetch(lat, lon).expect("Failed to fetch weather data");
    let air_quality_response = weather::fetch_air_quality(lat, lon).expect("Failed to fetch air quality data");

    println!("{}", display::output(open_meteo_response, air_quality_response, Local::now(), ip_info, config));
}