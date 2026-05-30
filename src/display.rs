use crate::weather;
use crate::location;
use crate::config;

use chrono::prelude::*;
use colored::Colorize;
use dirs;

struct Graphics {
    clear: String,
    partly_cloudy: String,
    cloudy: String,
    raining: String,
    thunderstorm: String,
    snow_hail: String,
}

impl Graphics {
    fn load() -> Self {
        let ascii_path = dirs::config_dir().unwrap().join("weatherfetch/ascii/");
        Graphics {
            clear: std::fs::read_to_string(ascii_path.join("clear.txt")).expect("Failed to read ascii/clear.txt"),
            partly_cloudy: std::fs::read_to_string(ascii_path.join("partly_cloudy.txt")).expect("Failed to read ascii/partly_cloudy.txt"),
            cloudy: std::fs::read_to_string(ascii_path.join("cloudy.txt")).expect("Failed to read ascii/cloudy.txt"),
            raining: std::fs::read_to_string("src/ascii/raining.txt").expect("Failed to read ascii/raining.txt"),
            thunderstorm: std::fs::read_to_string("src/ascii/thunderstorm.txt").expect("Failed to read ascii/thunderstorm.txt"),
            snow_hail: std::fs::read_to_string("src/ascii/snow_hail.txt").expect("Failed to read ascii/snow_hail.txt"),
        }
    }
}

fn c_to_f(celsius: f64) -> f64 {
    (celsius * 9.0/5.0) + 32.0
}
fn kmph_to_mph(kmph: f64) -> f64 {
    kmph * 0.621371
}
fn deg_to_compass(degrees: f64) -> &'static str {
    let directions = ["N", "NE", "E", "SE", "S", "SW", "W", "NW"];
    let index = ((degrees / 45.0).round() as usize) % 8;
    directions[index]
}
fn aqi_to_description(aqi: f64) -> &'static str {
    match aqi as u16 {
        0..=50 => "Good",
        51..=100 => "Fair",
        101..=150 => "Poor",
        151..=200 => "Very unhealthy",
        201..=250 => "Dangerous",
        _ => "Good luck.",
    }
}
fn visibility_to_description(visibility: f64) -> &'static str {
    match visibility as u32 {
        0..=500 => "Dense fog",
        501..=1000 => "Fog",
        1001..=2000 => "Heavy mist",
        2001..=4000 => "Haze",
        4001..=10000 => "Light haze",
        10001..=35000 => "Clear",
        35001..=70000 => "Very clear",
        _ => "THE SKY IS FALLING.",
    }
}
fn humidity_to_description(humidity: f64) -> &'static str {
    match humidity as u32 {
        0..=30 => "Dry",
        31..=60 => "Average",
        61..=100 => "Humid",
        _ => "Your flesh will no longer be needed.",
    }
}
fn kmph_to_description(kmph: f64) -> &'static str {
    match kmph as u32 {
        0..=10 => "Calm",
        11..=20 => "Breezy",
        21..=30 => "Strong breeze",
        31..=60 => "Windy",
        61..=80 => "High Wind",
        81..=102 => "Severe/Extreme winds",
        _ => "God have mercy on your soul.",
    }
}
fn cloudcover_to_description(cloud_cover: f64) -> &'static str {
    match cloud_cover as u32 {
        0..=10 => "Clear",
        11..=25 => "Mostly Clear",
        26..=50 => "Partly Cloudy",
        51..=84 => "Mostly Cloudy",
        85..=100 => "Overcast",
        _ => "He came early. He came late. He came when you least expected it.",
    }
}
fn cloudcover_to_consistent(cloud_cover: f64) -> &'static str {
    match cloud_cover as u32 {
        0..=10 =>   "    Clear    ",
        11..=25 =>  "Mostly Clear ",
        26..=50 =>  "Partly Cloudy",
        51..=84 =>  "Mostly Cloudy",
        85..=100 => "  Overcast   ",
        _ =>        "?????????????",
    }
}
fn should_you_bring_sunscreen(uv_index: f64) -> &'static str {
    match uv_index as u32 {
        0..=2 => "Maybe",
        3..=5 => "Probably",
        6..=7 => "Yes",
        8..=10 => "Absolutely",
        _ => "Stay inside. Run if you must. The children won't make it.",
    }
}
pub fn output(weather_data: weather::OpenMeteoResponse, air_quality_data: weather::AirQuality, forecast_data: weather::Forecast, weather_code_data: weather::WeatherCodeResponse, local_time: DateTime<Local>, ip_info: location::IPInfo, config: config::Config) -> String {

    let mut data = String::new();
    let mut output = String::new();
    let mut heart_string = String::new();
    let mut final_string = String::new();

    let forecast = forecast_data.daily;

    let graphics = Graphics::load();

    let time_index = local_time.hour() as usize;
    let sunrise = weather_data.daily.sunrise[0].split('T').nth(1).unwrap_or(&weather_data.daily.sunrise[0]);
    let sunset = weather_data.daily.sunset[0].split('T').nth(1).unwrap_or(&weather_data.daily.sunset[0]);

    let header_length: usize;
    let mut max_line_length: usize = 0;

    let icon: String; // oh my god this should have been a match statement but im lazy as fuck lmao :sob: 
    if weather_code_data.current.code >= 50 && weather_code_data.current.code <= 55 {
        icon = graphics.raining;
    } else if weather_code_data.current.code >= 58 && weather_code_data.current.code <= 67 {
        icon = graphics.raining;
    } else if weather_code_data.current.code >= 80 && weather_code_data.current.code <= 82 {
        icon = graphics.raining;
    } else if weather_code_data.current.code >= 91 && weather_code_data.current.code <= 92 {
        icon = graphics.raining;
    } else if weather_code_data.current.code >= 95 && weather_code_data.current.code <= 96 {
        icon = graphics.thunderstorm;
    } else if weather_code_data.current.code >= 56 && weather_code_data.current.code <= 57 {
        icon = graphics.snow_hail;
    } else if weather_code_data.current.code >= 71 && weather_code_data.current.code <= 77 {
        icon = graphics.snow_hail;
    } else if weather_code_data.current.code >= 83 && weather_code_data.current.code <= 90 {
        icon = graphics.snow_hail;
    } else if weather_code_data.current.code >= 93 && weather_code_data.current.code <= 94 {
        icon = graphics.snow_hail;
    } else {
        if weather_data.current.sky_condition_num <= 25 as f64 {
            icon = graphics.clear;
        } else if weather_data.current.sky_condition_num <= 50 as f64 {
            icon = graphics.partly_cloudy;
        } else {
            icon = graphics.cloudy;
        }
    }

    if !config.hide_location {
        if config.use_color {
            data.push_str(&format!("{}{} {} {} {}", ip_info.city.bold().bright_cyan(), ",".bold().bright_cyan(), ip_info.region.bold().bright_cyan(), "@", local_time.format("%H:%M").to_string().bold().bright_cyan()));
        } else {
            data.push_str(&format!("{} {} {} {} {}", ip_info.city, ",", ip_info.region, "@", local_time.format("%H:%M").to_string()));
        }
        header_length = format!("{}, {} @ {}", ip_info.city, ip_info.region, local_time.format("%H:%M")).len();
    } else {
        if config.use_color {
            data.push_str(&format!("{} {} {}", "Location, Hidden".bold().bright_cyan(), "@", local_time.format("%H:%M").to_string().bold().bright_cyan()));
        } else {
            data.push_str(&format!("{} {} {}", "Location, Hidden", "@", local_time.format("%H:%M").to_string()));
        }
        header_length = 24;
    }
    data.push_str("\n");
    for _ in 0..header_length {
        data.push('-');
    }
    data.push_str("\n\n");

    if config.use_color {
        if !config.use_imperial {
            data.push_str(&format!("{} {}°C\n", "Temperature:".bold().bright_cyan(), weather_data.current.temperature));
            data.push_str(&format!("    {} {}°C\n", "- Feels Like:".bold().bright_cyan(), weather_data.current.feels_like));
            data.push_str(&format!("    {} {}°C\n", "- Today's High:".bold().bright_cyan(), weather_data.daily.temperature_max[0]));
            data.push_str(&format!("    {} {}°C\n", "- Today's Low:".bold().bright_cyan(), weather_data.daily.temperature_min[0]));
            data.push_str(&format!("    {} {}\n", "- UV Index:".bold().bright_cyan(), weather_data.daily.uv_index[0]));
            data.push_str(&format!("        {} {}\n", "- Should you bring sunscreen?".bold().bright_cyan(), should_you_bring_sunscreen(weather_data.daily.uv_index[0])));
            data.push_str(&format!("{} {}\n", "Sky Condition:".bold().bright_cyan(), cloudcover_to_description(weather_data.current.sky_condition_num)));
            data.push_str(&format!("    {} {}%\n", "- Cloud Cover:".bold().bright_cyan(), weather_data.current.sky_condition_num));
            data.push_str(&format!("{} {}\n", "Wind:".bold().bright_blue(), kmph_to_description(weather_data.current.wind_speed)));
            data.push_str(&format!("    {} {} km/h\n", "- Speed:".bold().bright_cyan(), weather_data.current.wind_speed));
            data.push_str(&format!("    {} {} ({}°)\n", "- Direction:".bold().bright_cyan(), deg_to_compass(weather_data.current.wind_direction), weather_data.current.wind_direction));
            data.push_str(&format!("{} {}\n", "Humidity:".bold().bright_cyan(), humidity_to_description(weather_data.current.humidity)));
            data.push_str(&format!("    {} {}%\n", "- Relative Humidity:".bold().bright_cyan(), weather_data.current.humidity));
            data.push_str(&format!("    {} {}°C\n", "- Dew Point:".bold().bright_cyan(), weather_data.hourly.dew_point[time_index]));
            data.push_str(&format!("{} {}%\n", "Chance of Precipitation:".bold().bright_cyan(), weather_data.daily.precipitation_probability[0]));
            data.push_str(&format!("{} {}\n", "Visibility:".bold().bright_cyan(), visibility_to_description(weather_data.hourly.visibility[time_index])));
            data.push_str(&format!("    {} {} km\n", "- Visibility Distance:".bold().bright_cyan(), weather_data.hourly.visibility[time_index] / 1000.0));
            data.push_str(&format!("{} {}\n", "Air Quality:".bold().bright_cyan(), aqi_to_description(air_quality_data.current.aqi)));
            data.push_str(&format!("    {} {}\n", "- US AQI:".bold().bright_cyan(), air_quality_data.current.aqi));
            data.push_str(&format!("{} {}\n", "Today's sunrise:".bold().bright_cyan(), sunrise));
            data.push_str(&format!("{} {}\n\n", "Today's sunset:".bold().bright_cyan(), sunset));
        } else {
            data.push_str(&format!("{} {}°F\n", "Temperature:".bold().bright_cyan(), c_to_f(weather_data.current.temperature).round()));
            data.push_str(&format!("    {} {}°F\n", "- Feels Like:".bold().bright_cyan(), c_to_f(weather_data.current.feels_like).round()));
            data.push_str(&format!("    {} {}°F\n", "- Today's High:".bold().bright_cyan(), c_to_f(weather_data.daily.temperature_max[0]).round()));
            data.push_str(&format!("    {} {}°F\n", "- Today's Low:".bold().bright_cyan(), c_to_f(weather_data.daily.temperature_min[0]).round()));
            data.push_str(&format!("    {} {}\n", "- UV Index:".bold().bright_cyan(), weather_data.daily.uv_index[0]));
            data.push_str(&format!("        {} {}\n", "- Should you bring sunscreen?".bold().bright_cyan(), should_you_bring_sunscreen(weather_data.daily.uv_index[0])));
            data.push_str(&format!("{} {}\n", "Sky Condition:".bold().bright_cyan(), cloudcover_to_description(weather_data.current.sky_condition_num)));
            data.push_str(&format!("    {} {}%\n", "- Cloud Cover:".bold().bright_cyan(), weather_data.current.sky_condition_num));
            data.push_str(&format!("{} {}\n", "Wind:".bold().bright_cyan(), kmph_to_description(weather_data.current.wind_speed)));
            data.push_str(&format!("    {} {} mph\n", "- Speed:".bold().bright_cyan(), kmph_to_mph(weather_data.current.wind_speed).round()));
            data.push_str(&format!("    {} {} ({}°)\n", "- Direction:".bold().bright_cyan(), deg_to_compass(weather_data.current.wind_direction), weather_data.current.wind_direction));
            data.push_str(&format!("{} {}\n", "Humidity:".bold().bright_cyan(), humidity_to_description(weather_data.current.humidity)));
            data.push_str(&format!("    {} {}%\n", "- Relative Humidity:".bold().bright_cyan(), weather_data.current.humidity));
            data.push_str(&format!("    {} {}°F\n", "- Dew Point:".bold().bright_cyan(), c_to_f(weather_data.hourly.dew_point[time_index]).round()));
            data.push_str(&format!("{} {}%\n", "Chance of Precipitation:".bold().bright_cyan(), weather_data.daily.precipitation_probability[0]));
            data.push_str(&format!("{} {}\n", "Visibility:".bold().bright_cyan(), visibility_to_description(weather_data.hourly.visibility[time_index])));
            data.push_str(&format!("    {} {} miles\n", "- Visibility Distance:".bold().bright_cyan(), kmph_to_mph(weather_data.hourly.visibility[time_index] / 1000.0).round()));
            data.push_str(&format!("{} {}\n", "Air Quality:".bold().bright_cyan(), aqi_to_description(air_quality_data.current.aqi)));
            data.push_str(&format!("    {} {}\n", "- US AQI:".bold().bright_cyan(), air_quality_data.current.aqi));
            data.push_str(&format!("{} {}\n", "Today's sunrise:".bold().bright_cyan(), sunrise));
            data.push_str(&format!("{} {}\n\n", "Today's sunset:".bold().bright_cyan(), sunset));
        }
    } else {
        if !config.use_imperial {
            data.push_str(&format!("Temperature: {}°C\n", weather_data.current.temperature));
            data.push_str(&format!("    - Feels Like: {}°C\n", weather_data.current.feels_like));
            data.push_str(&format!("    - Today's High: {}°C\n", weather_data.daily.temperature_max[0]));
            data.push_str(&format!("    - Today's Low: {}°C\n", weather_data.daily.temperature_min[0]));
            data.push_str(&format!("    - UV Index: {}\n", weather_data.daily.uv_index[0]));
            data.push_str(&format!("        - Should you bring sunscreen? {}\n", should_you_bring_sunscreen(weather_data.daily.uv_index[0])));
            data.push_str(&format!("Sky Condition: {}\n", cloudcover_to_description(weather_data.current.sky_condition_num)));
            data.push_str(&format!("    - Cloud Cover: {}%\n", weather_data.current.sky_condition_num));
            data.push_str(&format!("Wind: {}\n", kmph_to_description(weather_data.current.wind_speed)));
            data.push_str(&format!("    - Speed: {} km/h\n", weather_data.current.wind_speed));
            data.push_str(&format!("    - Direction: {} ({}°)\n", deg_to_compass(weather_data.current.wind_direction), weather_data.current.wind_direction));
            data.push_str(&format!("Humidity: {}\n", humidity_to_description(weather_data.current.humidity)));
            data.push_str(&format!("    - Relative Humidity: {}%\n", weather_data.current.humidity));
            data.push_str(&format!("    - Dew Point: {}°C\n", weather_data.hourly.dew_point[time_index]));
            data.push_str(&format!("Chance of Precipitation: {}%\n", weather_data.daily.precipitation_probability[0]));
            data.push_str(&format!("Visibility: {}\n", visibility_to_description(weather_data.hourly.visibility[time_index])));
            data.push_str(&format!("    - Visibility Distance: {} km\n", weather_data.hourly.visibility[time_index] / 1000.0));
            data.push_str(&format!("Air Quality: {}\n", aqi_to_description(air_quality_data.current.aqi)));
            data.push_str(&format!("    - US AQI: {}\n", air_quality_data.current.aqi));
            data.push_str(&format!("Today's sunrise: {}\n", sunrise));
            data.push_str(&format!("Today's sunset: {}\n\n", sunset));
        } else {
            data.push_str(&format!("Temperature: {}°F\n", c_to_f(weather_data.current.temperature).round()));
            data.push_str(&format!("    - Feels Like: {}°F\n", c_to_f(weather_data.current.feels_like).round()));
            data.push_str(&format!("    - Today's High: {}°F\n", c_to_f(weather_data.daily.temperature_max[0]).round()));
            data.push_str(&format!("    - Today's Low: {}°F\n", c_to_f(weather_data.daily.temperature_min[0]).round()));
            data.push_str(&format!("    - UV Index: {}\n", weather_data.daily.uv_index[0]));
            data.push_str(&format!("        - Should you bring sunscreen? {}\n", should_you_bring_sunscreen(weather_data.daily.uv_index[0])));
            data.push_str(&format!("Sky Condition: {}\n", cloudcover_to_description(weather_data.current.sky_condition_num)));
            data.push_str(&format!("    - Cloud Cover: {}%\n", weather_data.current.sky_condition_num));
            data.push_str(&format!("Wind: {}\n", kmph_to_description(weather_data.current.wind_speed)));
            data.push_str(&format!("    - Speed: {} mph\n", kmph_to_mph(weather_data.current.wind_speed).round()));
            data.push_str(&format!("    - Direction: {} ({}°)\n", deg_to_compass(weather_data.current.wind_direction), weather_data.current.wind_direction));
            data.push_str(&format!("Humidity: {}\n", humidity_to_description(weather_data.current.humidity)));
            data.push_str(&format!("    - Relative Humidity: {}%\n", weather_data.current.humidity));
            data.push_str(&format!("    - Dew Point: {}°F\n", c_to_f(weather_data.hourly.dew_point[time_index]).round()));
            data.push_str(&format!("Chance of Precipitation: {}%\n", weather_data.daily.precipitation_probability[0]));
            data.push_str(&format!("Visibility: {}\n", visibility_to_description(weather_data.hourly.visibility[time_index])));
            data.push_str(&format!("    - Visibility Distance: {} miles\n", kmph_to_mph(weather_data.hourly.visibility[time_index] / 1000.0).round()));
            data.push_str(&format!("Air Quality: {}\n", aqi_to_description(air_quality_data.current.aqi)));
            data.push_str(&format!("    - US AQI: {}\n", air_quality_data.current.aqi));
            data.push_str(&format!("Today's sunrise: {}\n", sunrise));
            data.push_str(&format!("Today's sunset: {}\n\n", sunset));
        }
    }

    for (_index, line) in data.lines().enumerate() {
        let ascii_line: String;
        if config.use_color {
            ascii_line = icon.lines().nth(_index).unwrap_or("").bold().truecolor(4, 244, 214).to_string();
        } else {
            ascii_line = icon.lines().nth(_index).unwrap_or("").to_string();
        }
        if !config.show_forecast {
            if config.no_icon {
                output.push_str(&format!("  {}\n", line));
            } else {
                output.push_str(&format!("  {}  {}\n", ascii_line, line));
            }
        } else {
            if config.no_icon {
                output.push_str(&format!("  {}\n", line));
            } else {
                output.push_str(&format!("                   {}  {}\n", ascii_line, line));
            }
        }
        if line.len() > max_line_length {
            max_line_length = line.len();
        }
    }

    let mut forecast_string = String::new();
    forecast_string.push_str(&format!("      Today            Tomorrow         Overmorrow         In 3 Days         In 4 Days         In 5 Days         In 6 Days  \n"));
             forecast_string.push_str("╔═════════════════╦═════════════════╦═════════════════╦═════════════════╦═════════════════╦═════════════════╦═════════════════╗\n");
    forecast_string.push_str(&format!("║  {}  ║  {}  ║  {}  ║  {}  ║  {}  ║  {}  ║  {}  ║\n",
        cloudcover_to_consistent(forecast.cloud_cover_mean[0]),
        cloudcover_to_consistent(forecast.cloud_cover_mean[1]),
        cloudcover_to_consistent(forecast.cloud_cover_mean[2]),
        cloudcover_to_consistent(forecast.cloud_cover_mean[3]),
        cloudcover_to_consistent(forecast.cloud_cover_mean[4]),
        cloudcover_to_consistent(forecast.cloud_cover_mean[5]),
        cloudcover_to_consistent(forecast.cloud_cover_mean[6]),
    ));
    forecast_string.push_str("╟─────────────────╫─────────────────╫─────────────────╫─────────────────╫─────────────────╫─────────────────╫─────────────────╢\n");
    if config.use_imperial {
        for i in 0..7 {
            if c_to_f(forecast.temperature_max[i]).round() >= 100.0 {
                forecast_string.push_str(&format!("║ High:    {}°F ", c_to_f(forecast.temperature_max[i]).round()));
            } else if c_to_f(forecast.temperature_max[i]).round() < 10.0 && c_to_f(forecast.temperature_min[i]).round() > -10.0 {
                forecast_string.push_str(&format!("║ High:       {}°F ", c_to_f(forecast.temperature_max[i]).round()));
            } else {
                forecast_string.push_str(&format!("║ High:      {}°F ", c_to_f(forecast.temperature_max[i]).round()));
            }
        }
        forecast_string.push_str("║\n");
        for i in 0..7 {
            if c_to_f(forecast.temperature_min[i]).round() >= 100.0 {
                forecast_string.push_str(&format!("║ Low:      {}°F ", c_to_f(forecast.temperature_min[i]).round()));
            } else if c_to_f(forecast.temperature_min[i]).round() < 10.0 && c_to_f(forecast.temperature_min[i]).round() > -10.0 {
                forecast_string.push_str(&format!("║ Low:        {}°F ", c_to_f(forecast.temperature_min[i]).round()));
            } else {
                forecast_string.push_str(&format!("║ Low:       {}°F ", c_to_f(forecast.temperature_min[i]).round()));
            }
        }
        forecast_string.push_str("║\n");

    } else {
        for i in 0..7 {
            if forecast.temperature_max[i].round() >= 100.0 {
                forecast_string.push_str(&format!("║ High:    {}°C ", forecast.temperature_max[i].round()));
            } else if forecast.temperature_max[i].round() < 10.0 && forecast.temperature_min[i].round() > -10.0 {
                forecast_string.push_str(&format!("║ High:       {}°C ", forecast.temperature_max[i].round()));
            } else {
                forecast_string.push_str(&format!("║ High:      {}°C ", forecast.temperature_max[i].round()));
            }
        }
        forecast_string.push_str("║\n");
        for i in 0..7 {
            if forecast.temperature_min[i].round() >= 100.0 {
                forecast_string.push_str(&format!("║ Low:      {}°C ", forecast.temperature_min[i].round()));
            } else if forecast.temperature_min[i].round() < 10.0 && forecast.temperature_min[i].round() > -10.0 {
                forecast_string.push_str(&format!("║ Low:          {}°C ", forecast.temperature_min[i].round()));
            } else {
                forecast_string.push_str(&format!("║ Low:       {}°C ", forecast.temperature_min[i].round()));
            }
        }
        forecast_string.push_str("║\n");
    }
    forecast_string.push_str("╟─────────────────╫─────────────────╫─────────────────╫─────────────────╫─────────────────╫─────────────────╫─────────────────╢\n");
    for i in 0..7 {
        if forecast.precipitation_probability[i].round() == 100.0 {
            forecast_string.push_str(&format!("║ Precip:    100% "));
        } else if forecast.precipitation_probability[i].round() >= 10.0 {
            forecast_string.push_str(&format!("║ Precip:     {}% ", forecast.precipitation_probability[i].round()));
        } else {
            forecast_string.push_str(&format!("║ Precip:      {}% ", forecast.precipitation_probability[i].round()));
        }
    }
    forecast_string.push_str("║\n╟─────────────────╫─────────────────╫─────────────────╫─────────────────╫─────────────────╫─────────────────╫─────────────────╢\n");
    forecast_string.push_str(&format!("║ Sunrise:  {} ║ Sunrise:  {} ║ Sunrise:  {} ║ Sunrise:  {} ║ Sunrise:  {} ║ Sunrise:  {} ║ Sunrise:  {} ║\n", 
    forecast.sunrise[0].split('T').nth(1).unwrap_or(&forecast.sunrise[0]),
    forecast.sunrise[1].split('T').nth(1).unwrap_or(&forecast.sunrise[1]),
    forecast.sunrise[2].split('T').nth(1).unwrap_or(&forecast.sunrise[2]),
    forecast.sunrise[3].split('T').nth(1).unwrap_or(&forecast.sunrise[3]),
    forecast.sunrise[4].split('T').nth(1).unwrap_or(&forecast.sunrise[4]),
    forecast.sunrise[5].split('T').nth(1).unwrap_or(&forecast.sunrise[5]),
    forecast.sunrise[6].split('T').nth(1).unwrap_or(&forecast.sunrise[6])
    ));
    forecast_string.push_str(&format!("║ Sunset:   {} ║ Sunset:   {} ║ Sunset:   {} ║ Sunset:   {} ║ Sunset:   {} ║ Sunset:   {} ║ Sunset:   {} ║\n",
    forecast.sunset[0].split('T').nth(1).unwrap_or(&forecast.sunset[0]),
    forecast.sunset[1].split('T').nth(1).unwrap_or(&forecast.sunset[1]),
    forecast.sunset[2].split('T').nth(1).unwrap_or(&forecast.sunset[2]),
    forecast.sunset[3].split('T').nth(1).unwrap_or(&forecast.sunset[3]),
    forecast.sunset[4].split('T').nth(1).unwrap_or(&forecast.sunset[4]),
    forecast.sunset[5].split('T').nth(1).unwrap_or(&forecast.sunset[5]),
    forecast.sunset[6].split('T').nth(1).unwrap_or(&forecast.sunset[6])
    ));
    forecast_string.push_str("╚═════════════════╩═════════════════╩═════════════════╩═════════════════╩═════════════════╩═════════════════╩═════════════════╝\n");
    
    if !config.show_forecast {
        for _ in 0..max_line_length + 35 {
            heart_string.push_str("♡");
        }
    } else {
        heart_string.push_str("♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡");
    }

    if config.show_forecast {
        final_string.push_str(&format!("\n{}\n\n{}{}\n{}\n", heart_string, output, forecast_string, heart_string));
    } else {
        final_string.push_str(&format!("\n{}\n\n{}{}\n", heart_string, output, heart_string));
    }

    final_string
}